use serde::de::DeserializeOwned;
use toml::Value;

pub fn load_config<T: DeserializeOwned>(path: &str) -> T {
    let raw = std::fs::read_to_string(path)
        .expect(&format!("Failed to read config file: {}", path));

    toml::from_str(&raw)
        .expect(&format!("Failed to parse config file: {}", path))
}

pub fn load_with_overrides<T: DeserializeOwned>(path: &str, overrides: Option<&Value>) -> T {
    let raw = std::fs::read_to_string(path)
        .expect(&format!("Failed to read config file: {}", path));

    let mut value: Value = toml::from_str(&raw)
        .expect(&format!("Failed to parse config file: {}", path));

    if let Some(overrides) = overrides {
        merge(&mut value, overrides);
    }

    value.try_into()
        .expect(&format!("Failed to deserialize config file: {}", path))
}

pub fn load_raw(path: &str) -> Value {
    let raw = std::fs::read_to_string(path)
        .expect(&format!("Failed to read config file: {}", path));

    toml::from_str(&raw)
        .expect(&format!("Failed to parse config file: {}", path))
}

fn merge(base: &mut Value, overlay: &Value) {
    match (base, overlay) {
        (Value::Table(base), Value::Table(overlay)) => {
            for (key, value) in overlay {
                merge(
                    base.entry(key).or_insert(Value::Table(Default::default())),
                    value,
                );
            }
        }
        (base, overlay) => *base = overlay.clone(),
    }
}
