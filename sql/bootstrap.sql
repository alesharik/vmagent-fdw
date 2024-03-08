create foreign data wrapper vmagent_wrapper
  handler vmagent_fdw_handler
  validator vmagent_fdw_validator;

create server test_server
  foreign data wrapper vmagent_wrapper
  options (
    address 'http://127.0.0.1:8429/'
  );

create foreign table hello (
  state text not null,
  health text not null,
  last_samples_scraped bigint,
  last_scrape_duration double precision,
  last_scrape timestamp,
  last_error string,
  scrape_url string not null,
  scrape_pool string not null,
  labels jsonb,
  labels_instance string,
  discovered_labels jsonb
)
  server test_server;
