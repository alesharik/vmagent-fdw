# VMAgent Foreign Data Wrapper
Postgres extension to fetch data from vmagent.

### Setup
```sql
create extension vmagentfdw; -- load extension

create foreign data wrapper vmagent_wrapper
  handler vmagent_fdw_handler
  validator vmagent_fdw_validator; -- create FDW

create server test_server
  foreign data wrapper vmagent_wrapper
  options (
    address 'http://127.0.0.1:8429/' -- server address
  ); -- connect to prometheus server

create foreign table targets (
  state text not null,
  health text not null,
  last_samples_scraped bigint,
  last_scrape_duration double precision,
  last_scrape timestamp,
  last_error text,
  scrape_url text not null,
  scrape_pool text not null,
  labels jsonb,
  labels_job jsonb, -- map concrete label
  discovered_labels jsonb
)
  server test_server;
```