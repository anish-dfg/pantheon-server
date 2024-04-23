use super::{
    dto::{
        CreateDatasourceView, CreateDatasourceViewJob, CreateExportedUser, CreateJob, CreateJobWithDatasource,
        CreateUser, EditDatasourceView, EditJob, EditUser,
    },
    entities::{DatasourceView, DatasourceViewJob, DatasourceViewJobs, DatasourceViews, ExportedUser, Job, Jobs, User},
};
use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;

pub struct Sql {
    pub pool: PgPool,
}

impl Sql {
    pub async fn new(pg_uri: &str) -> Result<Self> {
        let sql = PgPoolOptions::new().max_connections(100).connect(pg_uri).await?;

        Ok(Self { pool: sql })
    }

    pub async fn create_or_fetch_user(&self, data: CreateUser) -> Result<String> {
        let mut txn = self.pool.begin().await?;
        let (user_id,) = sqlx::query_as::<_, (Uuid,)>(
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

    pub async fn edit_user(&self, user_id: Uuid, data: EditUser) -> Result<()> {
        let mut txn = self.pool.begin().await?;
        sqlx::query(
            "update users
            set first_name = $1, last_name = $2, email = $3, image_uri = $4
            where id = $5",
        )
        .bind(&data.first_name)
        .bind(&data.last_name)
        .bind(&data.email)
        .bind(&data.image_uri)
        .bind(user_id)
        .execute(&mut *txn)
        .await?;

        txn.commit().await?;

        Ok(())
    }

    pub async fn fetch_user(&self, user_id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "select id, first_name, last_name, email, image_uri, created_at, updated_at from users where id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn create_datasource_view(&self, data: CreateDatasourceView) -> Result<String> {
        let mut txn = self.pool.begin().await?;
        let (datasource_view_id,) = sqlx::query_as::<_, (Uuid,)>(
            "insert into datasource_views (user_id, view_name, datasource, description, metadata)
            values ($1, $2, $3, $4, $5)
            returning id",
        )
        .bind(data.user_id)
        .bind(&data.view_name)
        .bind(data.datasource)
        .bind(&data.description)
        .bind(&data.metadata)
        .fetch_one(&mut *txn)
        .await?;

        txn.commit().await?;

        Ok(datasource_view_id.to_string())
    }

    pub async fn edit_datasource_view(&self, datasource_view_id: Uuid, data: EditDatasourceView) -> Result<()> {
        let mut txn = self.pool.begin().await?;
        sqlx::query(
            "update datasource_views
            set view_name = $1, datasource = $2, description = $3, metadata = $4
            where id = $5",
        )
        .bind(&data.view_name)
        .bind(data.datasource)
        .bind(&data.description)
        .bind(&data.metadata)
        .bind(datasource_view_id)
        .execute(&mut *txn)
        .await?;

        txn.commit().await?;

        Ok(())
    }

    pub async fn delete_datasource_view(&self, datasource_view_id: Uuid) -> Result<()> {
        let mut txn = self.pool.begin().await?;
        sqlx::query("delete from datasource_views where id = $1")
            .bind(datasource_view_id)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;

        Ok(())
    }

    pub async fn fetch_datasource_view(&self, datasource_view_id: Uuid) -> Result<Option<DatasourceView>> {
        let datasource_view = sqlx::query_as::<_, DatasourceView>(
            "select id, created_at, updated_at, 
             user_id, view_name, datasource, description, metadata 
             from datasource_views where id = $1",
        )
        .bind(datasource_view_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(datasource_view)
    }

    pub async fn fetch_datasource_views(&self) -> Result<DatasourceViews> {
        let datasource_view = sqlx::query_as::<_, DatasourceView>(
            "select id, created_at, updated_at, 
             user_id, view_name, datasource, description, metadata from datasource_views",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(datasource_view)
    }

    // Job methods
    pub async fn create_job(&self, data: CreateJob) -> Result<String> {
        let mut txn = self.pool.begin().await?;
        let (job_id,) = sqlx::query_as::<_, (Uuid,)>(
            "insert into jobs (user_id, status, job_type, metadata)
            values ($1, $2, $3, $4)
            returning id",
        )
        .bind(data.user_id)
        .bind(data.status)
        .bind(data.job_type)
        .bind(&data.metadata)
        .fetch_one(&mut *txn)
        .await?;

        txn.commit().await?;

        Ok(job_id.to_string())
    }

    pub async fn create_job_with_datasource(&self, data: CreateJobWithDatasource) -> Result<(String, String)> {
        let mut txn = self.pool.begin().await?;
        let job_id = self.create_job(data.clone().try_into()?).await?;
        let (datasource_view_job_id,) = sqlx::query_as::<_, (Uuid,)>(
            "insert into datasource_view_jobs (job_id, datasource_view_id) 
             values ($1, $2) returning id",
        )
        .bind(Uuid::parse_str(&job_id)?)
        .bind(data.datasource_view_id)
        .fetch_one(&mut *txn)
        .await?;

        txn.commit().await?;
        Ok((job_id, datasource_view_job_id.into()))
    }

    pub async fn edit_job(&self, job_id: &str, data: EditJob) -> Result<()> {
        let mut txn = self.pool.begin().await?;
        sqlx::query(
            "update jobs
            set status = $1, job_type = $2, metadata = $3
            where id = $4",
        )
        .bind(data.status)
        .bind(data.job_type)
        .bind(&data.metadata)
        .bind(Uuid::parse_str(job_id)?)
        .execute(&mut *txn)
        .await?;

        txn.commit().await?;

        Ok(())
    }

    pub async fn delete_job(&self, job_id: &str) -> Result<()> {
        let mut txn = self.pool.begin().await?;
        sqlx::query("delete from jobs where id = $1")
            .bind(Uuid::parse_str(job_id)?)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;

        Ok(())
    }

    pub async fn fetch_job(&self, job_id: &str) -> Result<Option<Job>> {
        let job = sqlx::query_as::<_, Job>("select * from jobs where id = $1")
            .bind(Uuid::parse_str(job_id)?)
            .fetch_optional(&self.pool)
            .await?;

        Ok(job)
    }

    pub async fn fetch_jobs(&self) -> Result<Jobs> {
        let jobs = sqlx::query_as::<_, Job>(
            "select id, created_at, updated_at, user_id, status, job_type, metadata from jobs",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(jobs)
    }

    pub async fn mark_job_errored(&self, job_id: Uuid) -> Result<()> {
        let mut txn = self.pool.begin().await?;
        sqlx::query("update jobs set status='error'::job_status where id=$1")
            .bind(job_id)
            .execute(&mut *txn)
            .await?;
        txn.commit().await?;
        Ok(())
    }

    pub async fn mark_job_complete(&self, job_id: Uuid) -> Result<()> {
        let mut txn = self.pool.begin().await?;
        sqlx::query("update jobs set status='complete'::job_status where id=$1")
            .bind(job_id)
            .execute(&mut *txn)
            .await?;
        txn.commit().await?;
        Ok(())
    }

    // DatasourceViewJob methods
    pub async fn create_datasource_view_job(&self, data: CreateDatasourceViewJob) -> Result<String> {
        let mut txn = self.pool.begin().await?;
        let (datasource_view_job_id,) = sqlx::query_as::<_, (Uuid,)>(
            "insert into datasource_view_jobs (user_id, job_id)
            values ($1, $2) returning id",
        )
        .bind(data.user_id)
        .bind(data.job_id)
        .fetch_one(&mut *txn)
        .await?;

        txn.commit().await?;

        Ok(datasource_view_job_id.into())
    }

    pub async fn delete_datasource_view_job(&self, user_id: &str, job_id: &str) -> Result<()> {
        let mut txn = self.pool.begin().await?;
        sqlx::query(
            "delete from datasource_view_jobs
            where user_id = $1 and job_id = $2",
        )
        .bind(Uuid::parse_str(user_id)?)
        .bind(Uuid::parse_str(job_id)?)
        .execute(&mut *txn)
        .await?;

        txn.commit().await?;

        Ok(())
    }

    // ExportedUser methods
    pub async fn create_exported_user(&self, data: CreateExportedUser) -> Result<String> {
        let mut txn = self.pool.begin().await?;
        let (exported_user_id,) = sqlx::query_as::<_, (Uuid,)>(
            "insert into exported_users (first_name, last_name, email, exported_from)
            values ($1, $2, $3, $4)
            returning id",
        )
        .bind(&data.first_name)
        .bind(&data.last_name)
        .bind(&data.email)
        .bind(data.exported_from)
        .fetch_one(&mut *txn)
        .await?;

        txn.commit().await?;

        Ok(exported_user_id.to_string())
    }

    pub async fn delete_exported_user(&self, exported_user_id: &str) -> Result<()> {
        let mut txn = self.pool.begin().await?;
        sqlx::query("delete from exported_users where id = $1")
            .bind(Uuid::parse_str(exported_user_id)?)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;

        Ok(())
    }

    pub async fn fetch_exported_user(&self, exported_user_id: &str) -> Result<Option<ExportedUser>> {
        let exported_user = sqlx::query_as::<_, ExportedUser>(
            "select id, created_at, updated_at, job_id, first_name, last_name, email, exported_from 
            from exported_users where id = $1",
        )
        .bind(Uuid::parse_str(exported_user_id)?)
        .fetch_optional(&self.pool)
        .await?;
        Ok(exported_user)
    }

    pub async fn fetch_datasource_view_jobs(&self, datasource_view_id: Uuid) -> Result<Jobs> {
        let jobs = sqlx::query_as::<_, Job>(
            "select j.id, j.created_at, j.updated_at, j.user_id, j.status, j.job_type, j.metadata
             from jobs j
             join datasource_view_jobs dvj ON j.id = dvj.job_id
             where dvj.datasource_view_id=$1",
        )
        .bind(datasource_view_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(jobs)
    }
}
