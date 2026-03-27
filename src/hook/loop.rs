use crate::hook::Hook;
use crate::hook::format::{create_message, MarkdownFlavor};
use crate::loki::LokiStream;
use serde::Serialize;

#[derive(Serialize)]
struct LoopPayload {
    text: String,
}

pub struct LoopHook {
    webhook_url: String,
}

impl LoopHook {
    pub fn new(webhook_url: String) -> Self {
        Self { webhook_url }
    }
}

impl Hook for LoopHook {
    fn send(&self, loki_stream: &LokiStream, visible_labels: &[String], dc: &str) {
        let formatted_message = create_message(&MarkdownFlavor::Mattermost, loki_stream, visible_labels, dc);
        let client = reqwest::blocking::Client::new();
        let payload = LoopPayload {
            text: formatted_message,
        };
        client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::Server;

    #[test]
    fn test_send_to_loop() {
        let mut server = Server::new();
        let _m = server
            .mock("POST", "/hooks/test-webhook")
            .with_status(200)
            .create();
        let webhook_url = format!("{}/hooks/test-webhook", server.url());
        let hook = LoopHook::new(webhook_url);
        let empty_labels: Vec<String> = vec![];
        hook.send(&Default::default(), &empty_labels, "dc");
    }
}
