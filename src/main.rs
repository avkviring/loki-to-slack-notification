use loki_to_slack_notification::execute;
use loki_to_slack_notification::hook::Hook;
use loki_to_slack_notification::hook::slack::SlackHook;
use loki_to_slack_notification::hook::r#loop::LoopHook;
use std::env;
use std::fs;

fn main() {
    let (loki_url, queries_config, hooks, visible_labels, dc) = read_envs();
    execute(
        loki_url.as_str(),
        &hooks,
        queries_config.as_str(),
        visible_labels,
        dc.as_str(),
    );
}

fn read_envs() -> (String, String, Vec<Box<dyn Hook>>, Vec<String>, String) {
    let loki_url = env::var("LOKI_URL").expect("LOKI_URL must be set");
    let dc = env::var("DC").expect("DC (data center name) must be set");
    let config_map_path = env::var("CONFIGMAP_PATH").expect("CONFIGMAP_PATH must be set");
    let visible_labels = env::var("VISIBLE_LABELS")
        .expect("VISIBLE_LABELS must be set")
        .split(",")
        .map(|v| v.trim().to_string())
        .collect();

    let mut hooks: Vec<Box<dyn Hook>> = Vec::new();
    if let Ok(url) = env::var("SLACK_WEBHOOK_URL") {
        hooks.push(Box::new(SlackHook::new(url)));
    }
    if let Ok(url) = env::var("LOOP_WEBHOOK_URL") {
        hooks.push(Box::new(LoopHook::new(url)));
    }

    (
        loki_url,
        fs::read_to_string(config_map_path).unwrap(),
        hooks,
        visible_labels,
        dc,
    )
}
