use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub command: String
}


#[derive(Debug, serde::Serialize)]
pub struct HandlerSaveResponse {
    pub command_saved: bool,
    pub message: String,
    pub alert_icon_type: String,
    pub alert_title: String
}


#[derive(Serialize, Deserialize)]
pub struct LoginParams {
    pub username: String,
    pub password: String
}


#[derive(Serialize, Deserialize)]
pub struct SignupParams {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String
}


#[derive(Serialize, Deserialize)]
pub struct SignupResponse {
    pub signed_up: bool, 
    pub message: String,
    pub title: String,
    pub icon: String
}

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct UserCommand
{
    pub name: String,
    pub function: String,
}


#[derive(Serialize, Deserialize)]
pub struct CommandToParse {
    pub command: String,
    pub level: i32,
}


#[derive(Serialize, Deserialize)]
pub struct ParserResponse {
    pub error: bool,
    pub command: String,
    pub state: String,
    pub num_of_steps: i32
}


#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Field {
    pub x: i32,
    pub y: i32
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct LevelDimensions {
    pub width: i32,
    pub height: i32
}

#[derive(Serialize, Deserialize)]
pub struct Level {
    pub level: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LevelMap {
    pub dimensions: LevelDimensions,
    pub start: Field,
    pub finish: Field,
    pub obstacles: Vec<Field>,
    pub stars: Vec<Field>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct UserStats {
    pub num_of_steps: i32,
    pub level: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BestStats {
    pub username: String,
    pub num_of_steps: i32,
    pub level: i32,
}

#[derive(Deserialize)]
pub struct CommandDeleteRequest {
    pub command: String,
}
