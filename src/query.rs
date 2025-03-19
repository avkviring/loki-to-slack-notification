pub fn get_queries(config_content: &str) -> Vec<String> {
    config_content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("#"))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::query::get_queries;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_queries() {
        let mut file = NamedTempFile::new().unwrap();
        file.write(
            r#"
            {job="my-job"}
            {job="another-job"}
            # комментарий
            {app="my-app"} | logfmt
            "#
            .as_bytes(),
        )
        .unwrap();
        let config = std::fs::read_to_string(file.path()).unwrap();
        let queries = get_queries(config.as_str());
        assert_eq!(queries.len(), 3);
        assert_eq!(queries[0], "{job=\"my-job\"}");
        assert_eq!(queries[1], "{job=\"another-job\"}");
        assert_eq!(queries[2], "{app=\"my-app\"} | logfmt");
    }
}
