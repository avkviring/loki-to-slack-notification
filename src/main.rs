use loki_to_slack_notification::execute;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let (loki_url, queries_config, slack_webhook_url, visible_labels, dc) = read_envs();
    execute(loki_url.as_str(), slack_webhook_url.as_str(), queries_config.as_str(), visible_labels, dc.as_str())?;
    Ok(())
}


fn read_envs() -> (String, String, String, Vec<String>, String) {
    let loki_url = env::var("LOKI_URL").expect("LOKI_URL must be set");
    let dc = env::var("DC").expect("DC (data center name) must be set");
    let config_map_path = env::var("CONFIGMAP_PATH").expect("CONFIGMAP_PATH must be set");
    let slack_webhook_url = env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL must be set");
    let visible_labels = env::var("VISIBLE_LABELS")
        .expect("VISIBLE_LABELS must be set")
        .split(",")
        .map(|v| v.to_string())
        .map(|v| v.trim().to_string())
        .collect();
    (
        loki_url,
        fs::read_to_string(config_map_path).unwrap(),
        slack_webhook_url,
        visible_labels,
        dc
    )
}
