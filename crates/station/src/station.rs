use std::sync::{Arc, Mutex};

use crossbeam::channel::Receiver;
use robot::data::{transport::telemetry::Message, condition::RobotCondition};

use crate::{Gui, gui::scene::{CubeScene, Scene, StationaryScene}, data::transport::telemetry::Mapper};

pub struct Station {
    robot: Arc<Mutex<RobotCondition>>,
    gui: Gui,
}

impl Station {
    pub fn new(condition: Arc<Mutex<RobotCondition>>) -> Self {
        let initial_scene = Scene::Stationary(StationaryScene::new());
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

    pub async fn run(&mut self, telemetry: &Receiver<Message>) {
        println!("{:?}", self.robot);

        self.gui.run(Arc::clone(&self.robot), telemetry).await;
    }
}