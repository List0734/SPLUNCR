use std::sync::{Arc, Mutex};

use crossbeam::channel::Receiver;
use robot::data::{condition::RobotCondition, transport::telemetry::state::State};
use shared::data::transport::message::Message;

use crate::{Gui, data::transport::telemetry::Mapper, gui::scene::{ConnectingScene, CubeScene, Scene, StationaryScene}};

pub struct Station {
    robot: Arc<Mutex<RobotCondition>>,
    gui: Gui,
}

impl Station {
    pub fn new(condition: Arc<Mutex<RobotCondition>>) -> Self {
        let initial_scene = ConnectingScene::new();
        let gui = Gui::new(initial_scene);

        Self {
            robot: condition,
            gui
        }
    }

    /*
    pub fn update_condition(&mut self, condition: RobotCondition) {
        self.robot = condition;
    }
    */

    /*
    pub fn receive_message(&mut self, message: Message) {
        println!("{:?}", self.robot);
        Mapper::ingest(&mut self.robot, message);
    }
    */

   /* 
    pub fn receive_messages(&mut self, receiver: Receiver<Message>) {
        while let Ok(message) = receiver.try_recv() {
            station.receive_message(message);
        }
    }
    */

    pub async fn run(&mut self, telemetry: &Receiver<Message<State>>) {
        println!("{:?}", self.robot);

        self.gui.run(Arc::clone(&self.robot), telemetry).await;
    }
}