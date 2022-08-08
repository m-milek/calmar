pub struct Config {
    pub date_format: String,
    pub time_format: String,
    pub default_path: String,
    pub prompt_text: String,
    pub prompt_color: String,
    pub prompt_bold: bool,
    pub prompt_italic: bool,
    pub prompt_underline: bool
    //TODO: add fields
}

pub fn get_config() -> Config {
    Config {
        date_format: "DD/MM/YYYY".to_string(),
        time_format: "HH:MM".to_string(),
        default_path: "/home/michal/.calmar".to_string(),
	prompt_text: "[calmar]".to_string(),
	/*
	Available colors:
	- black
	- red
	- green
	- yellow
	- blue
	- magenta
	- cyan
	- white
	- bright_*
	*/
	prompt_color: "bright_white".to_string(),
	prompt_bold: true,
	prompt_italic: false,
	prompt_underline: false,
    }
    //TODO: find a config file in .config/calmar
    //TODO: config.json
    //TODO: read it into Config
    //TODO: lazy static it
}
