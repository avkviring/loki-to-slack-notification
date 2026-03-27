use crate::hook::Hook;
use crate::loki::{invoke_loki_get_api, LokiStream};
use crate::query::get_queries;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod loki;
pub mod hook;
pub mod query;

pub fn execute(
    loki_url: &str,
    hooks: &[Box<dyn Hook>],
    queries_config: &str,
    visible_labels: Vec<String>,
    dc: &str,
) {
    let queries = get_queries(queries_config);
    let streams = execute_query(loki_url, queries);
    for stream in streams {
        for hook in hooks {
            hook.send(&stream, &visible_labels, dc);
        }
    }
}

pub fn execute_query(loki_url: &str, queries: Vec<String>) -> Vec<LokiStream> {
    let mut streams = vec![];

    let to = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let from = to - 60 * 5 - 10;

    for query in queries {
        let result = invoke_loki_get_api(loki_url, &query, from, to);
        for stream in result.data.result {
            streams.push(stream);
        }
    }
    streams
}
