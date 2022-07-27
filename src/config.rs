pub struct Config {
    pub date_format: String,
    pub time_format: String,
    pub default_path: String
    //TODO: add fields
}

pub fn get_config() -> Config {
    Config {
        date_format: "DD/MM/YYYY".to_string(),
        time_format: "HH:MM".to_string(),
	default_path: "/home/michal/.calmar".to_string()
    }
    //TODO: find a config file in .config/calmar
    //TODO: config.json
    //TODO: read it into Config
    //TODO: lazy static it
}
