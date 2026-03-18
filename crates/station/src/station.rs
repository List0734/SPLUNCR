use std::sync::{Arc, Mutex};

use robot::data::condition::RobotCondition;

use crate::{Gui, data::{config::StationConfig, transport::{command::Commands, communication::Communication}}, gui::scene::ConnectingScene};

pub struct Station {
    robot: Arc<Mutex<RobotCondition>>,
    gui: Gui,
    commands: Commands,
    communication: Arc<Communication>,
}

impl Station {
    pub fn new(condition: Arc<Mutex<RobotCondition>>, config: StationConfig) -> Self {
        let communication = Arc::new(
            Communication::new(config.communication).expect("Error Opening Port")
        );
        communication.spawn_telemetry_receiver(Arc::clone(&condition));
        communication.spawn_command_connector();
        communication.spawn_video_receiver();

        let commands = Commands::new();
        let video = communication.video_frame();

        let initial_scene = ConnectingScene::new(Arc::clone(&communication));
        let robot_snapshot = condition.lock().unwrap().clone();
        let gui = Gui::new(initial_scene, &robot_snapshot, video);

        Self {
            robot: condition,
            gui,
            commands,
            communication,
        }
    }

    pub async fn run(&mut self) {
        while let Some(message) = self.commands.receive() {
            let bytes = bincode::serialize(&message).unwrap();
            let _ = self.communication.send_command(&bytes);
        }

        self.gui.run(Arc::clone(&self.robot)).await;
    }
}
