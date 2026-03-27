use crate::loki::LokiStream;

pub enum MarkdownFlavor {
    Slack,
    Mattermost,
}

pub fn create_message(
    flavor: &MarkdownFlavor,
    loki_stream: &LokiStream,
    visible_labels: &[String],
    dc: &str,
) -> String {
    let bold = |text: &str| match flavor {
        MarkdownFlavor::Slack => format!("*{}*", text),
        MarkdownFlavor::Mattermost => format!("**{}**", text),
    };

    let mut message = String::new();
    message.push_str(&format!("Datacenter {}\n", bold(dc)));
    visible_labels.iter().for_each(|label| {
        if let Some(value) = loki_stream.stream.get(label) {
            message.push_str(&format!("{} `{}`\n", bold(label), value));
        }
    });
    loki_stream.values.iter().for_each(|item| {
        message.push_str(&format!("```{}```\n", item.1.replace("`", "~")));
    });
    message
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::loki::LokiStream;
    use std::collections::HashMap;

    fn test_stream() -> LokiStream {
        LokiStream {
            stream: HashMap::from([
                ("pod".to_string(), "podA".to_string()),
                ("namespace".to_string(), "production".to_string()),
            ]),
            values: vec![
                ("time".to_string(), "message1".to_string()),
                ("time".to_string(), "message2`1".to_string()),
            ],
        }
    }

    #[test]
    fn test_slack_flavor() {
        let labels = vec!["pod".to_string(), "namespace".to_string()];
        let message = create_message(&MarkdownFlavor::Slack, &test_stream(), &labels, "dc");
        assert_eq!(
            "Datacenter *dc*\n*pod* `podA`\n*namespace* `production`\n```message1```\n```message2~1```\n",
            message
        );
    }

    #[test]
    fn test_mattermost_flavor() {
        let labels = vec!["pod".to_string(), "namespace".to_string()];
        let message = create_message(&MarkdownFlavor::Mattermost, &test_stream(), &labels, "dc");
        assert_eq!(
            "Datacenter **dc**\n**pod** `podA`\n**namespace** `production`\n```message1```\n```message2~1```\n",
            message
        );
    }
}
