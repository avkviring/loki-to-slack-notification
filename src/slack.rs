use crate::loki::LokiStream;
use slack_hook::{PayloadBuilder, Slack};
use std::error::Error;

pub fn send_to_slack(
    webhook_url: &str,
    loki_stream: &LokiStream,
    visible_labels: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let formatted_message = create_message(loki_stream, visible_labels);
    let slack = Slack::new(webhook_url)?;
    let payload = PayloadBuilder::new().text(formatted_message).build()?;
    slack.send(&payload)?;
    Ok(())
}

fn create_message(loki_stream: &LokiStream, visible_labels: &[String]) -> String {
    let mut message = String::new();
    visible_labels.iter().for_each(|label| {
        if let Some(value) = loki_stream.stream.get(label) {
            message.push_str(format!("*{}* `{}`\n", label, value).as_str());
        }
    });
    loki_stream.values.iter().for_each(|item| {
        message.push_str(format!("```{}```\n", item.1.replace("`", "~")).as_str())
    });
    message
}

#[cfg(test)]
mod test {
    use crate::loki::LokiStream;
    use crate::slack::{create_message, send_to_slack};
    use mockito::Server;
    use std::collections::HashMap;

    #[test]
    fn test_send_to_slack() {
        let mut server = Server::new();
        let _m = server
            .mock("POST", "/services/your/webhook/url")
            .with_status(200)
            .create();
        let webhook_url = format!("{}/services/your/webhook/url", server.url());
        let result = send_to_slack(&webhook_url, &Default::default(), &Default::default());
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_message() {
        let stream = LokiStream {
            stream: HashMap::from([
                ("pod".to_string(), "podA".to_string()),
                ("namespace".to_string(), "production".to_string()),
            ]),
            values: vec![
                ("time".to_string(), "message1".to_string()),
                ("time".to_string(), "message2`1".to_string()),
            ],
        };

        let labels = vec!["pod".to_string(), "namespace".to_string()];
        let message = create_message(&stream, &labels);
        assert_eq!(
            "*pod* `podA`\n*namespace* `production`\n```message1```\n```message2~1```\n",
            message
        );
    }
}
