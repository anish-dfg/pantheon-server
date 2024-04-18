pub mod entities;
mod errors;

use anyhow::{bail, Result};
use mobc::Pool;
use mobc_redis::redis::AsyncCommands;
use mobc_redis::{redis, RedisConnectionManager};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use uuid::Uuid;

use self::entities::{
    CreateDatasourceView, CreateJob, CreateUser, DatasourceView, EditUser, Job, MarkJobErrored, User,
};

pub struct Sql {
    pub pg: PgPool,
}

pub type RedisPool = Pool<RedisConnectionManager>;

pub struct Cache {
    pub redis: RedisPool,
}

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

impl Cache {
    pub fn new(cache_url: &str) -> Result<Self> {
        let client = redis::Client::open(cache_url)?;
        let manager = RedisConnectionManager::new(client);
        let pool = Pool::builder()
            .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
            .max_open(CACHE_POOL_MAX_OPEN)
            .max_idle(CACHE_POOL_MAX_IDLE)
            .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
            .build(manager);
        Ok(Cache { redis: pool })
    }
    pub async fn set_json<'a, T>(&self, key: &str, value: T) -> Result<()>
    where
        T: Serialize + Deserialize<'a>,
    {
        let mut conn = self.redis.get().await?;
        let bytes = serde_json::to_string(&value).map(|s| s.as_bytes().to_vec())?;

        conn.set(key.to_owned(), bytes).await?;

        Ok(())
    }

    pub async fn get_json<'a, T>(&self, key: &str) -> Result<Option<T>>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut conn = self.redis.get().await?;
        let json: Option<String> = conn.get(key).await?;

        if let Some(json) = json {
            let s: T = serde_json::from_str(&json).map_err(|e| {
                log::warn!("{}", e.to_string());
                e
            })?;
            Ok(Some(s))
        } else {
            Ok(None)
        }
    }

    pub async fn evict(&self, key: &str) -> Result<()> {
        let mut conn = self.redis.get().await?;
        conn.del(key).await?;
        Ok(())
    }
}

impl Sql {
    pub async fn new(pg_uri: &str) -> Result<Self> {
        let pg = PgPoolOptions::new().max_connections(100).connect(pg_uri).await?;

        Ok(Self { pg })
    }

    pub async fn create_or_fetch_user(&self, data: CreateUser) -> Result<String> {
        let mut txn = self.pg.begin().await?;
        let (user_id,): (Uuid,) = sqlx::query_as(
            "with inserted_user as (
                insert into users (first_name, last_name, email, image_uri)
                values ($1, $2, $3, $4)
                on conflict (email) do nothing
                returning id
            )
            select id from inserted_user
            union all
            select id from users where email = $3",
        )
        .bind(&data.first_name)
        .bind(&data.last_name)
        .bind(&data.email)
        .bind(&data.image_uri)
        .fetch_one(&mut *txn)
        .await?;

        txn.commit().await?;

        Ok(user_id.into())
    }

    pub async fn edit_user(&self, data: EditUser) -> Result<()> {
        let mut txn = self.pg.begin().await?;
        sqlx::query("update users set first_name = $1, last_name = $2, image_uri = $3 where email = $4")
            .bind(&data.first_name)
            .bind(&data.last_name)
            .bind(&data.image_uri)
            .bind(&data.email)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;

        Ok(())
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let mut txn = self.pg.begin().await?;
        let Some(user): Option<User> = sqlx::query_as(
            "select id, created_at, updated_at, first_name, last_name, email, image_uri
                from users
                where email = $1",
        )
        .bind(email)
        .fetch_optional(&mut *txn)
        .await?
        else {
            bail!("unable to read row from database")
        };

        txn.commit().await?;

        Ok(user.into())
    }

    pub async fn create_datasource_view(&self, data: CreateDatasourceView) -> Result<String> {
        let mut txn = self.pg.begin().await?;
        let (datasource_view_id,): (Uuid,) = sqlx::query_as(
            "insert into datasource_views (user_id, description, datasource_name, view_name, metadata) values ($1, $2, $3, $4, $5) returning id",
        )
        .bind(Uuid::parse_str(&data.user_id)?)
        .bind(&data.description)
        .bind(&data.datasource_name)
        .bind(&data.view_name)
        .bind(&data.metadata)
        .fetch_one(&mut *txn)
        .await?;

        txn.commit().await?;
        Ok(datasource_view_id.into())
    }

    pub async fn fetch_datasource_views(&self) -> Result<Vec<DatasourceView>> {
        let mut txn = self.pg.begin().await?;
        let data: Vec<DatasourceView> = sqlx::query_as(
            "select id, user_id, description, created_at, updated_at, view_name, datasource_name, metadata from datasource_views",
        )
        .fetch_all(&mut *txn)
        .await?;
        txn.commit().await?;
        Ok(data)
    }

    pub async fn fetch_datasource_view_by_id(&self, id: &str) -> Result<Option<DatasourceView>> {
        let mut txn = self.pg.begin().await?;
        let data: Option<DatasourceView> = sqlx::query_as(
            "select id, user_id, description, created_at, updated_at, view_name, datasource_name, metadata from datasource_views where id = $1",
        )
            .bind(Uuid::parse_str(id)?)
        .fetch_optional(&mut *txn)
        .await?;
        txn.commit().await?;
        Ok(data)
    }

    pub async fn create_job(&self, data: CreateJob) -> Result<String> {
        let mut txn = self.pg.begin().await?;
        let (job_id,): (Uuid,) =
            sqlx::query_as("insert into jobs (user_id, description, status) values ($1, $2, $3) returning id")
                .bind(Uuid::parse_str(&data.user_id)?)
                .bind(&data.description)
                .bind("pending")
                .fetch_one(&mut *txn)
                .await?;
        txn.commit().await?;
        Ok(job_id.into())
    }

    pub async fn mark_job_complete(&self, job_id: &str) -> Result<()> {
        let mut txn = self.pg.begin().await?;

        let _ = sqlx::query("update jobs set status=$1 where id=$2")
            .bind("complete")
            .bind(Uuid::parse_str(job_id)?)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;
        Ok(())
    }

    pub async fn mark_job_errored(&self, data: MarkJobErrored) -> Result<()> {
        let mut txn = self.pg.begin().await?;

        let _ = sqlx::query("update jobs set status=$1 where id=$2")
            .bind("error")
            .bind(Uuid::parse_str(&data.job_id)?)
            .execute(&mut *txn)
            .await?;

        let _ = sqlx::query("insert into job_errors (job_id, error_data) values ($1, $2)")
            .bind(Uuid::parse_str(&data.job_id)?)
            .bind(&data.error)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;
        Ok(())
    }

    pub async fn fetch_jobs(&self) -> Result<Vec<Job>> {
        let mut txn = self.pg.begin().await?;
        let jobs: Vec<Job> =
            sqlx::query_as("select id, user_id, description, status, created_at, updated_at from jobs")
                .fetch_all(&mut *txn)
                .await?;

        txn.commit().await?;
        Ok(jobs)
    }
}
