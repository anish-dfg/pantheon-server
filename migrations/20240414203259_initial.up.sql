-- Add up migration script here

begin;
create extension if not exists "uuid-ossp";
--
create type job_status as enum('pending', 'error', 'complete');
create type job_type as enum('export_users', 'import_data');
create type supported_datasource as enum('airtable', 'google_workspace_admin_directory');
--
create or replace function update_timestamp()
  returns trigger
  as $$
begin
  new.updated_at = current_timestamp;
  return new;
end;
$$
language plpgsql;
--
-- users table
create table if not exists users(
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  first_name text not null,
  last_name text not null,
  email text not null,
  image_uri text not null,
  unique (email)
);
create or replace trigger update_users_timestamp
  before update on users for each row
  execute function update_timestamp();
--
-- datasources table
create table if not exists datasource_views (
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  user_id uuid not null references users(id) on delete cascade,
  view_name text not null,
  datasource supported_datasource not null,
  description text not null,
  metadata jsonb not null default '{}'::jsonb
);
create or replace trigger update_datasources_timestamp
  before update on datasource_views for each row
  execute function update_timestamp();
--
--
create table if not exists jobs (
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  user_id uuid not null references users(id) on delete cascade,
  status job_status not null default 'pending'::job_status,
  job_type job_type not null,
  metadata jsonb not null default '{}'::jsonb
);
create or replace trigger update_jobs_timestamp 
  before update on jobs for each row
  execute function update_timestamp();
--
--
create table if not exists datasource_view_jobs (
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  datasource_view_id uuid not null references datasource_views(id) on delete cascade,
  job_id uuid not null references jobs(id) on delete cascade
);
create or replace trigger update_datasource_view_jobs 
  before update on datasource_view_jobs for each row
  execute function update_timestamp();
--
--
create table if not exists exported_users (
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  job_id uuid not null references jobs(id) on delete cascade,
  first_name text not null,
  last_name text not null,
  email text not null,
  exported_from supported_datasource not null
);
create or replace trigger update_exported_users_timestamp 
  before update on exported_users for each row
  execute function update_timestamp();
--
--
commit;
