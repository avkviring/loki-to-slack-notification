use crate::loki::LokiStream;

pub mod format;
pub mod slack;
pub mod r#loop;

pub trait Hook {
    fn send(&self, loki_stream: &LokiStream, visible_labels: &[String], dc: &str);
}
