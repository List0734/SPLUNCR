use serde::Deserialize;

use crate::platform::F;

#[derive(Clone, Debug, Deserialize)]
pub struct ThrusterConfig {
    pub name: String,
    pub placement: Placement,
    //pub characteristics: 
}

#[derive(Clone, Debug, Deserialize)]
pub struct Placement {
    pub position: [F; 3],
    pub direction: [F; 3],
}

#[derive(Clone, Debug, Deserialize)]
pub struct ThrustCharacteristics {

}