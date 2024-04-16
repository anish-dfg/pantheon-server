-- Add down migration script here
drop table if exists users cascade;
drop table if exists datasources cascade;
drop function if exists create_timestamp;
drop extension if exists "uuid-ossp";
