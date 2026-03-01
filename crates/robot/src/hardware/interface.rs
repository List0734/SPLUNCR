pub mod motor;

use motor::Motor;

pub trait Hal {
    type Motor: Motor;
    //type Imu: Imu;
    //type DepthSensor: DepthSensor;
}