use egui::{Context, Ui};
use robot::data::condition::RobotCondition;

use super::{config_screen, state_screen};

#[derive(PartialEq)]
pub enum TreeView {
    State,
    Config,
}

impl TreeView {
    fn label(&self) -> &'static str {
        match self {
            TreeView::State => "Robot State",
            TreeView::Config => "Robot Config",
        }
    }
}

pub fn data_panel(ctx: &Context, open: &mut bool, selected: &mut TreeView, robot: &RobotCondition) {
    if !*open {
        return;
    }

    egui::SidePanel::right("data_panel")
        .default_width(300.0)
        .resizable(true)
        .show(ctx, |ui| {
            draw_selector(ui, selected);
            ui.separator();
            match selected {
                TreeView::State => state_screen(ui, &robot.state),
                TreeView::Config => config_screen(ui, &robot.config),
            }
        });
}

fn draw_selector(ui: &mut Ui, selected: &mut TreeView) {
    ui.horizontal(|ui| {
        ui.label("View:");
        egui::ComboBox::from_id_salt("tree_selector")
            .selected_text(selected.label())
            .show_ui(ui, |ui| {
                ui.selectable_value(selected, TreeView::State, "Robot State");
                ui.selectable_value(selected, TreeView::Config, "Robot Config");
            });
    });
}
