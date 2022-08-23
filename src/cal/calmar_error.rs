pub enum CalmarError {
    OpenFile { e: std::io::Error },
    ParseJSON { e: serde_json::Error },
    WriteFile { e: std::io::Error },
}
