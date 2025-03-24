pub fn get_queries(config_content: &str) -> Vec<String> {
    let lines: Vec<String> = config_content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("#"))
        .collect();

    let variables: Vec<Vec<&str>> = lines
        .iter()
        .filter(|line| line.starts_with("$"))
        .map(|line| line[1..].split("=").clone().collect::<Vec<&str>>())
        .collect();
    let mut query = vec![];
    lines
        .iter()
        .filter(|line| !line.starts_with("$"))
        .for_each(|line| {
            let mut line = line.to_string();
            variables.iter().for_each(|var| {
                let name = format!("${}$", var[0]);
                line = line.replace(name.as_str(), &var[1]);
            });
            query.push(line);
        });
    query
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
            $value=super-job
            {job="$value$"}
            {job="another-job"}
            # комментарий
            {app="my-app"} | logfmt
            "#
            .as_bytes(),
        )
        .unwrap();
        let config = std::fs::read_to_string(file.path()).unwrap();
        let queries = get_queries(config.as_str());
        println!("queries: {:?}", queries);
        assert_eq!(queries.len(), 3);
        assert_eq!(queries[0], "{job=\"super-job\"}");
        assert_eq!(queries[1], "{job=\"another-job\"}");
        assert_eq!(queries[2], "{app=\"my-app\"} | logfmt");
    }
}
