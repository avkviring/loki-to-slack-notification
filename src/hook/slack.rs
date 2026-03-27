use crate::hook::Hook;
use crate::hook::format::{create_message, MarkdownFlavor};
use crate::loki::LokiStream;
use slack_hook::{PayloadBuilder, Slack};

pub struct SlackHook {
    webhook_url: String,
}

impl SlackHook {
    pub fn new(webhook_url: String) -> Self {
        Self { webhook_url }
    }
}

impl Hook for SlackHook {
    fn send(&self, loki_stream: &LokiStream, visible_labels: &[String], dc: &str) {
        let formatted_message = create_message(&MarkdownFlavor::Slack, loki_stream, visible_labels, dc);
        let slack = Slack::new(self.webhook_url.as_str()).unwrap();
        let payload = PayloadBuilder::new()
            .text(formatted_message)
            .build()
            .unwrap();
        slack.send(&payload).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::Server;

    #[test]
    fn test_send_to_slack() {
        let mut server = Server::new();
        let _m = server
            .mock("POST", "/services/your/webhook/url")
            .with_status(200)
            .create();
        let webhook_url = format!("{}/services/your/webhook/url", server.url());
        let hook = SlackHook::new(webhook_url);
        let empty_labels: Vec<String> = vec![];
        hook.send(&Default::default(), &empty_labels, "dc");
    }
}
