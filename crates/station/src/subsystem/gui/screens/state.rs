use egui::{CollapsingHeader, Ui};
use robot::data::state::RobotState;
use serde_json::Value;

// Recursive function to render any serde_json::Value as a tree
fn render_value_recursive(ui: &mut Ui, name: &str, value: &Value) {
    match value {
        Value::Object(map) => {
            CollapsingHeader::new(name).show(ui, |ui| {
                let mut complex: Vec<_> = map.iter().filter(|(_, v)| matches!(v, Value::Object(_) | Value::Array(_))).collect();
                let mut simple: Vec<_> = map.iter().filter(|(_, v)| !matches!(v, Value::Object(_) | Value::Array(_))).collect();
                complex.sort_by_key(|(k, _)| *k);
                simple.sort_by_key(|(k, _)| *k);

                for (k, v) in complex {
                    render_value_recursive(ui, k, v);
                }
                for (k, v) in simple {
                    render_value_recursive(ui, k, v);
                }
            });
        }
        Value::Array(arr) => {
            CollapsingHeader::new(name).show(ui, |ui| {
                for (i, v) in arr.iter().enumerate() {
                    render_value_recursive(ui, &i.to_string(), v);
                }
            });
        }
        Value::String(s) => { ui.label(format!("{}: {}", name, s)); }
        Value::Number(n) => { ui.label(format!("{}: {}", name, n)); }
        Value::Bool(b) => { ui.label(format!("{}: {}", name, b)); }
        Value::Null => { ui.label(format!("{}: null", name)); }
    }
}

pub fn state_screen(ui: &mut Ui, state: &RobotState) {
    // Serialize into json
    let value = serde_json::to_value(state).unwrap();

    egui::ScrollArea::vertical()
        .max_height(ui.available_height())
        .show(ui, |ui| {
            render_value_recursive(ui, "RobotState", &value);
        });
}