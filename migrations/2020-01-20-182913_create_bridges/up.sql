drop table if exists bridges;
create table bridges (
    slug varchar not null primary key,
    target_url varchar not null,
    active boolean not null default 't'
)
