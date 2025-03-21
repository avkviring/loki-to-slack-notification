use crate::loki::{invoke_loki_get_api, LokiStream};
use crate::query::get_queries;
use crate::slack::send_to_slack;
use serde::Deserialize;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod loki;
pub mod slack;

pub mod query;

pub fn execute(
    loki_url: &str,
    slack_webhook_url: &str,
    queries_config: &str,
    visible_labels: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let queries = get_queries(queries_config);
    let streams = execute_query(&loki_url, queries)?;
    for stream in streams {
        send_to_slack(slack_webhook_url, &stream, &visible_labels)?;
    }
    Ok(())
}

pub fn execute_query(loki_url: &str, queries: Vec<String>) -> Result<Vec<LokiStream>, Box<dyn Error>> {
    let mut streams = vec![];

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let two_minutes_ago = now - 120;

    for query in queries {
        let result = invoke_loki_get_api(&loki_url, &query, two_minutes_ago, now);
        for stream in result.data.result {
            streams.push(stream);
        }
    }
    Ok(streams)
}