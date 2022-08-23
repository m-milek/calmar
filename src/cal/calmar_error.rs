pub enum CalmarError {
    ReadFile { e: std::io::Error },
    ParseJSON { e: serde_json::Error },
    WriteFile { e: std::io::Error },
    CreateFile {e: std::io::Error },
    ToJSON { e: serde_json::Error },
    ActiveCalendarCount {e: usize},    
}
