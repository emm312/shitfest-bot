use lazy_static::lazy_static;

lazy_static! {
    static ref CONFIG_FILE: String = std::fs::read_to_string("Secrets.toml").unwrap();
}

pub fn get_db_url() -> String {
    let config: toml::Value = toml::from_str(&CONFIG_FILE).unwrap();
    config["db_url"].as_str().unwrap().to_string()
}

pub fn get_bot_token() -> String {
    let config: toml::Value = toml::from_str(&CONFIG_FILE).unwrap();
    config["token"].as_str().unwrap().to_string()
}

pub fn get_db_name() -> String {
    let config: toml::Value = toml::from_str(&CONFIG_FILE).unwrap();
    config["db_name"].as_str().unwrap().to_string()
}

pub fn get_db_username() -> String {
    let config: toml::Value = toml::from_str(&CONFIG_FILE).unwrap();
    config["username"].as_str().unwrap().to_string()
}

pub fn get_db_password() -> String {
    let config: toml::Value = toml::from_str(&CONFIG_FILE).unwrap();
    config["db_password"].as_str().unwrap().to_string()
}
