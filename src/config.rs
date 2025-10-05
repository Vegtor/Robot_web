use serde::{Deserialize, Serialize};
use std::env;


#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseConfig{
    pub uri: String,
    pub user: String,
    pub password: String,
    pub hash_seed: String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct Config{
    pub db: DatabaseConfig
}


impl Default for Config{
    fn default() -> Self {
        let mut uri: String = String::from("");
        let mut user: String = String::from("");
        let mut password: String = String::from("");
        let mut hash_seed: String = String::from("");

        match dotenvy::dotenv() {
            Ok(path) => println!(".env read successfully from {}", path.display()),
            Err(e) => {
                println!("Could not load .env file: {e}");
                panic!("No configuration file found!")
            }
        }
 

        match env::var("URI") {
            Ok(s) => uri = s.clone(),
            Err(e) => println!("{}",e)
        }

        match env::var("USER") {
            Ok(s) => user = s.clone(),
            Err(e) => println!("{}",e)
        }

        match env::var("PASSWORD") {
            Ok(s) => password = s.clone(),
            Err(e) => println!("{}",e)
        }

        match env::var("PASSWORD") {
            Ok(s) => hash_seed = s.clone(),
            Err(e) => println!("{}",e)
        }

        Self{db: DatabaseConfig { uri, user, password, hash_seed }}
    }
}