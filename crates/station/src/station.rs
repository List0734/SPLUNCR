use std::sync::{Arc, Mutex};

use crossbeam::channel::Receiver;
use robot::data::{condition::RobotCondition, transport::telemetry::state::State};
use shared::data::transport::message::Message;

use crate::{Gui, data::transport::{communication::{self, Communication}, telemetry::Mapper}, gui::scene::{ConnectingScene, CubeScene, Scene, StationaryScene}};

pub struct Station {
    robot: Arc<Mutex<RobotCondition>>,
    gui: Gui,
    communication: Arc<Communication>,
}

impl Station {
    pub fn new(condition: Arc<Mutex<RobotCondition>>) -> Self {
        let initial_scene = ConnectingScene::new();
        let gui = Gui::new(initial_scene);

        let communication = Arc::new(Communication::new("0.0.0.0:9001").expect("Error Opening Port"));
        communication.spawn_telemetry_receiver(Arc::clone(&condition));

        Self {
            robot: condition,
            gui,
            communication,
        }
    }
    
    pub async fn run(&mut self) {
        self.gui.run(Arc::clone(&self.robot)).await;
    }
}