CREATE TABLE bridges (
    id SERIAL PRIMARY KEY,
    slug VARCHAR NOT NULL,
    target_url VARCHAR NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 't'
)
