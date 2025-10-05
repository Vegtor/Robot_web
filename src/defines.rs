
pub enum RobotState {
    Finished,
    Crashed,
    Moving,
    OutOfFuel
}


pub trait RobotStateToString {
    // Required method
    fn to_string(&self) -> String;
}


impl RobotStateToString for RobotState {
    fn to_string(&self) -> String {
        match self {
            RobotState::Crashed => return "Crashed".to_string(),
            RobotState::Finished => return "Finished".to_string(),
            RobotState::Moving => return "Moving".to_string(),
            RobotState::OutOfFuel => return "OutOfFuel".to_string()
        }
    }
}