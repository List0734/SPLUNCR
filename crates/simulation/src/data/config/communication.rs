use serde::{Deserialize, Serialize};

use robot::data::condition::config::CommunicationConfig;
use station::data::config::StationCommunicationConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimCommunicationConfig {
    pub robot: CommunicationConfig,
    pub station: StationCommunicationConfig,
}
