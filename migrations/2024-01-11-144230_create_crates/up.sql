CREATE TABLE crates (
  id SERIAL PRIMARY KEY,
  rustaceans_id integer NOT NULL REFERENCES rustaceans(id),
  code varchar(64) NOT NULL,
  name VARCHAR(128) NOT NULL,
  version VARCHAR(64) NOT NULL,
  description text,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
)