-- Add up migration script here

begin;
create extension if not exists "uuid-ossp";

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
  datasource_name text not null,
  description text not null,
  metadata jsonb not null default '{}'::jsonb
);
create or replace trigger update_datasources_timestamp
  before update on datasource_views for each row
  execute function update_timestamp();
commit;

