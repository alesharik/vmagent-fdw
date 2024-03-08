use crate::error::{Error, Result};
use std::collections::HashMap;
use pgrx::JsonB;
use supabase_wrappers::prelude::*;
use crate::client::{Client, Target};
use crate::utils::from_chrono;

enum TargetItem {
    Active(Target),
    Dropped(Target),
}

impl TargetItem {
    pub fn target(&self) -> &Target {
        match self {
            TargetItem::Active(s) => s,
            TargetItem::Dropped(s) => s,
        }
    }

    pub fn state(&self) -> String {
        match self {
            TargetItem::Active(_) => "active",
            TargetItem::Dropped(_) => "dropped",
        }.to_string()
    }
}

#[wrappers_fdw(
    version = "0.1.0",
    author = "alesharik",
    website = "https://github.com/alesharik/vmagent_fdw",
    error_type = "Error"
)]
pub(crate) struct VmagentFdw {
    client: Client,
    tgt_columns: Vec<Column>,
    result: Vec<TargetItem>,
    idx: usize,
}

impl ForeignDataWrapper<Error> for VmagentFdw {
    fn new(options: &HashMap<String, String>) -> Result<Self> {
        Ok(Self {
            client: Client::from_str(options.get("address").ok_or(Error::AddressOptionRequired)?),
            tgt_columns: vec![],
            idx: 0,
            result: vec![],
        })
    }

    fn begin_scan(
        &mut self,
        _quals: &[Qual],
        columns: &[Column],
        _sorts: &[Sort],
        _limit: &Option<Limit>,
        _options: &HashMap<String, String>,
    ) -> Result<()> {
        let req = self.client.targets()?;
        self.result = req.active.into_iter()
            .map(|x| TargetItem::Active(x))
            .chain(req.dropped.into_iter().map(|x| TargetItem::Dropped(x)))
            .collect();
        self.tgt_columns = columns.to_vec();
        self.idx = 0;
        Ok(())
    }

    fn iter_scan(&mut self, row: &mut Row) -> Result<Option<()>> {
        if self.idx >= self.result.len() {
            return Ok(None)
        }

        let data: &TargetItem = &self.result[self.idx];
        for col in &self.tgt_columns {
            match col.name.as_str() {
                "state" => row.push(&col.name, Some(Cell::String(data.state()))),
                "health" => row.push(&col.name, Some(Cell::String(data.target().health.to_string()))),
                "last_samples_scraped" => row.push(&col.name, Some(Cell::I64(data.target().last_samples_scraped as i64))),
                "last_scrape_duration" => row.push(&col.name, Some(Cell::F64(data.target().last_scrape_duration))),
                "last_scrape" => row.push(&col.name, data.target().last_scrape.map(|t| Cell::Timestamp(from_chrono(t)))),
                "last_error" => row.push(&col.name, if data.target().last_error.is_empty() { None } else { Some(Cell::String(data.target().last_error.clone())) }),
                "scrape_url" => row.push(&col.name, Some(Cell::String(data.target().scrape_url.clone()))),
                "scrape_pool" => row.push(&col.name, Some(Cell::String(data.target().scrape_pool.clone()))),
                "labels" => row.push(&col.name, Some(Cell::Json(JsonB(serde_json::to_value(&data.target().labels)?)))),
                "discovered_labels" => row.push(&col.name, Some(Cell::Json(JsonB(serde_json::to_value(&data.target().discovered_labels)?)))),
                _ => if col.name.starts_with("labels_") {
                    row.push(&col.name, col.name.strip_prefix("labels_")
                        .and_then(|key| data.target().labels.get(key))
                        .map(|s| Cell::String(s.to_string())));
                } else if col.name.starts_with("discovered_labels_") {
                    row.push(&col.name, col.name.strip_prefix("discovered_labels_")
                        .and_then(|key| data.target().discovered_labels.get(key))
                        .map(|s| Cell::String(s.to_string())));
                } else {
                    row.push(&col.name, None)
                },
            }
        }
        self.idx += 1;
        Ok(Some(()))
    }

    fn end_scan(&mut self) -> Result<()> {
        self.result.clear();
        Ok(())
    }
}

