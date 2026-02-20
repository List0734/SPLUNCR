use egui::{CollapsingHeader, Ui};
use robot::data::condition::{ConfigBundle, StateBundle, config::Config};
use serde_json::Value;

// Recursive function to render any serde_json::Value as a tree
fn render_value(ui: &mut Ui, name: &str, value: &Value) {
    match value {
        Value::Object(map) => {
            CollapsingHeader::new(name).show(ui, |ui| {
                for (k, v) in map {
                    render_value(ui, k, v);
                }
            });
        }
        Value::Array(arr) => {
            CollapsingHeader::new(name).show(ui, |ui| {
                for (i, v) in arr.iter().enumerate() {
                    render_value(ui, &i.to_string(), v);
                }
            });
        }
        Value::String(s) => { ui.label(format!("{}: {}", name, s)); }
        Value::Number(n) => { ui.label(format!("{}: {}", name, n)); }
        Value::Bool(b) => { ui.label(format!("{}: {}", name, b)); }
        Value::Null => { ui.label(format!("{}: null", name)); }
    }
}

pub fn config_screen(ui: &mut Ui, config: &ConfigBundle) {
    // Serialize into json
    let value = serde_json::to_value(config).unwrap();

    egui::ScrollArea::vertical()
        .max_height(ui.available_height())
        .show(ui, |ui| {
            render_value(ui, "ConfigBundle", &value);
        });
}