-- Add up migration script here
create table if not exists jobs (
  id uuid not null default uuid_generate_v4() primary key, 
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  user_id uuid not null references users(id) on delete cascade,
  description text not null,
  status text not null
);
create or replace trigger update_jobs_timestamp
  before update on jobs for each row
  execute function update_timestamp();
  --
  --
create table if not exists job_errors (
  id uuid not null default uuid_generate_v4() primary key,
  created_at timestamptz not null default current_timestamp,
  updated_at timestamptz not null default current_timestamp,
  job_id uuid not null references jobs(id) on delete cascade,
  error_data jsonb not null default '{}'::jsonb
);
create or replace trigger update_job_error_timestamp
  before update on job_errors for each row
  execute function update_timestamp();

