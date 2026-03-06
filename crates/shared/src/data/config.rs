use serde::de::DeserializeOwned;

pub fn load_config<T: DeserializeOwned>(path: &str) -> T {
    let raw = std::fs::read_to_string(path)
        .expect(&format!("Failed to read config file: {}", path));

    toml::from_str(&raw)
        .expect(&format!("Failed to parse config file: {}", path))
}
