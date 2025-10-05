extern crate neo4rs;

use core::num;

use neo4rs::{Graph, Result, RowStream};

use crate::config;
use crate::models::{UserCommand, Field, LevelDimensions, UserStats, BestStats};

use argon2::{self, Config, Variant, Version};


pub struct Database{
    pub graph: Graph,
    pub hash_seed: String
}


impl Database{

    pub async fn new(db_config: config::Config) -> Result<Self> {
        Ok(Self {
            graph: Graph::new(db_config.db.uri, db_config.db.user, db_config.db.password)
                .await
                .unwrap(),
            hash_seed: db_config.db.hash_seed,
        })
    }


    pub async fn query(&self, query: String) -> Result<RowStream>
    {
        self.graph.execute(neo4rs::query(query.as_str())).await
    }


    pub async fn add_command(&self, command: &str, command_name: &str, username: &str) -> bool
    {
        //let query: String = String::from("MATCH (user:USER {username: \"") + username + "\"}) -[:OWNS]-> (commands:COMMAND) WITH user, COUNT(commands) as numCommands 
        //CREATE (user) -[:OWNS]-> (:COMMAND {name: \"" + command_name + "\", function: \"" + command + "\", id: randomUUID()})";

        let command_check = String::from("MATCH (user:USER {username: \"") + username + "\"}) -[:OWNS]-> (commands:COMMAND {name: \"" + command_name + "\"}) 
        return toString(COUNT(commands)) as numOfCommands";

        let mut result = self.query(command_check).await.unwrap();

        while let Ok(Some(row)) = result.next().await {
            let number_of_commands = row.get::<String>("numOfCommands").unwrap();

            println!("{}", number_of_commands);

            if number_of_commands.parse::<i32>().unwrap() != 0 {
                return false;
            }
        }

        let query: String = String::from("MATCH (user:USER {username: \"") + username + "\"}) 
        CREATE (user) -[:OWNS]-> (:COMMAND {name: \"" + command_name + "\", function: \"" + command + "\", id: randomUUID()})";

        match self.run_query(query).await {
            Ok(()) => return true,
            Err(_e) => return false,
        };

        //true
    }


    pub async fn get_user_commands(&self, username: &str) -> Result<Vec<UserCommand>>
    {
        let query = format!(
            "MATCH (user:USER {{username: \"{}\"}}) -[:OWNS]-> (c:COMMAND) RETURN c.name AS name, c.function AS command",
            username
        );

        let mut result = match self.query(query).await {
            Ok(row) => row,
            Err(e) => return Err(e),
        };

        let mut response = Vec::<UserCommand>::new();

        while let Ok(Some(row)) = result.next().await {
            let name = row.get::<String>("name").unwrap();
            let function = row.get::<String>("command").unwrap();

            let user_command = UserCommand { name, function };

            response.push(user_command);
        }

        Ok(response)
    }


    pub async fn register_user(&self, username: &str, password: &str) -> bool
    {
        let salt = self.hash_seed.as_bytes();
        let config = Config {
            variant: Variant::Argon2i,
            version: Version::Version13,
            mem_cost: 65536,
            time_cost: 1,
            lanes: 4,
            secret: &[],
            ad: &[],
            hash_length: 32
        };
        let hashed_password = argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap();


        let query: String = String::from("CREATE (:USER {username: ") + "\"" + username + "\", password: \"" + hashed_password.as_str() + "\"})";

        //self.run_query(query).await;
        match self.run_query(query).await {
            Ok(()) => return true,
            Err(_e) => return false,
        };
    }


    pub async fn login_user(&self, username: &str, password: &str) -> bool
    {
        let query: String = String::from("MATCH (u:USER {username: ") + "\"" + username + "\"}) return u.password as password";
        let mut result = self.query(query).await.unwrap();

        while let Ok(Some(row)) = result.next().await {
            let db_password = row.get::<String>("password").unwrap();

            let config = Config {
                variant: Variant::Argon2i,
                version: Version::Version13,
                mem_cost: 65536,
                time_cost: 1,
                lanes: 4,
                secret: &[],
                ad: &[],
                hash_length: 32
            };
            let hashed_password = argon2::hash_encoded(password.as_bytes(), self.hash_seed.as_bytes(), &config).unwrap();

            //let matches: bool = argon2::verify_encoded(&db_password, password.as_bytes()).unwrap();
            let matches = hashed_password.as_str() == db_password.as_str();
            if matches == true {
                return true;
            }
        }

        false
    }


    pub async fn delete_command(&self, username: &str, command_name: &str) -> bool
    {
        let query: String = String::from("MATCH (:USER {username: ") + "\"" + username + "\"}) -[:OWNS]-> (command:COMMAND {name: \"" + command_name + "\"}) detach delete command";

        //self.run_query(query).await.unwrap();
        match self.run_query(query).await {
            Ok(()) => return true,
            Err(_e) => return false,
        };
    }


    pub async fn add_solved_level(&self, index: i32, num_of_steps: i32, username: &str) -> bool 
    {
        let query: String = String::from("MATCH (user:USER {username: \"") + username + "\"}) MATCH (level:LEVEL {index:" + index.to_string().as_str() + "})
        CREATE (user) -[:SOLVED {num_steps: " + num_of_steps.to_string().as_str() + "}]-> (level)";

        //self.run_query(query).await.unwrap();
        match self.run_query(query).await {
            Ok(()) => return true,
            Err(_e) => return false,
        };
    }


    pub async fn find_best(&self) -> Result<Vec<Vec<BestStats>>>
    {
        let query: String = String::from("MATCH (usr:USER) -[s:SOLVED]-> (lvl:LEVEL)
        WITH usr.username as user, toString(lvl.index) as level, toString(s.num_steps) as number_of_steps
        return user, level, number_of_steps");

        //self.graph.execute(neo4rs::query(query.as_str())).await
        let mut result = match self.query(query).await {
            Ok(row) => row,
            Err(e) => return Err(e),
        };


        let mut result_vec = Vec::<BestStats>::new();

        while let Ok(Some(row)) = result.next().await {
            let user = row.get::<String>("user").unwrap();
            let num_of_steps = row.get::<String>("number_of_steps").unwrap().parse().unwrap();
            let index = row.get::<String>("level").unwrap().parse().unwrap();

            let beststats = BestStats{username: user, num_of_steps: num_of_steps, level: index};

            result_vec.push(beststats);
        }


        result_vec.sort_by(|a, b| {
            let level = a.level.cmp(&b.level);
            if level == std::cmp::Ordering::Equal {
                a.num_of_steps.cmp(&b.num_of_steps)
            } else {
                level
            }
        });


        let mut tables = Vec::<Vec<BestStats>>::new();

        if !result_vec.clone().is_empty() {
            let mut prev_index = result_vec.clone()[0].level;
            let mut help_vec = Vec::<BestStats>::new();
            for stats in result_vec {
                if stats.level == prev_index {
                    prev_index = stats.level;
                    help_vec.push(stats);
                }
                else {
                    prev_index = stats.level;
                    tables.push(help_vec.clone());
                    help_vec.clear();
                    help_vec.push(stats);
                }
            }
            tables.push(help_vec.clone());
        }

        println!("{:?}", tables);

        Ok(tables)
    }


    pub async fn get_user_best_stats(&self, username: &str) -> Result<Vec<UserStats>>
    {
        let query: String = String::from("MATCH (u:USER {username: '") + username + "'}) -[s:SOLVED]->(l:LEVEL) RETURN toString(MIN(s.num_steps)) AS number_of_steps,toString(l.index) AS level";

        println!("{}", query.clone());

        let mut result = match self.query(query).await {
            Ok(row) => row,
            Err(e) => return Err(e),
        };


        let mut result_vec = Vec::<UserStats>::new();


        while let Ok(Some(row)) = result.next().await {
            let num_of_steps = row.get::<String>("number_of_steps").unwrap().parse().unwrap();
            let index = row.get::<String>("level").unwrap().parse().unwrap();

            let userstats = UserStats {num_of_steps: num_of_steps, level: index};

            result_vec.push(userstats);
        }

        Ok(result_vec)
    }


    pub async fn add_level(&self, index: i32, width: i32, height: i32) -> bool
    {
        let query: String = String::from("CREATE (:LEVEL {index: ") + index.to_string().as_str() + ", width: " + width.to_string().as_str() + ", height: " + height.to_string().as_str() + "})";

        //self.run_query(query).await.unwrap();
        match self.run_query(query).await {
            Ok(()) => return true,
            Err(_e) => return false,
        };
    }


    pub async fn add_level_with_fields(&self, username: &str, index: i32, dimensions: LevelDimensions, start: Field, finish: Field, obstacles: Vec<Field>, stars: Vec<Field>) -> bool
    {
        if start.clone().x == -1 || finish.clone().x == -1 {
            return false;
        }


        let mut field_types = String::from("[");
        let mut x_coords = String::from("[");
        let mut y_coords = String::from("[");
        for field in obstacles.clone() {
            field_types += "'obstacle', ";
            x_coords += (field.x.to_string() + ", ").as_str();
            y_coords += (field.y.to_string() + ", ").as_str();
        }

        for star in stars.clone() {   //Odkomentuje se, aby se pridali stars do fieldtypes
            field_types += "'star', ";
            x_coords += (star.x.to_string() + ", ").as_str();
            y_coords += (star.y.to_string() + ", ").as_str();
        }

        field_types.pop(); field_types.pop();
        x_coords.pop(); x_coords.pop();
        y_coords.pop(); y_coords.pop();
        field_types += "] as fieldTypes, ";
        x_coords += "] as Xs, ";
        y_coords += "] as Ys ";

        /*if obstacles.is_empty() {
            field_types = "[] as fieldTypes, ".to_string();
            x_coords = "[] as Xs, ".to_string();
            y_coords = "[] as Ys ".to_string();
        }*/

        if obstacles.is_empty() && stars.is_empty() {     //Horni podminka se zameni s touto, jelikoz budeme pridavat i hvezdy
            field_types = "[] as fieldTypes, ".to_string();
            x_coords = "[] as Xs, ".to_string();
            y_coords = "[] as Ys ".to_string();
        }



        let start = String::from("CREATE (lvl) -[:CONTAINS]->(:FIELD {type: 'start', x: ") +
            start.x.to_string().as_str() + ", y: " + start.y.to_string().as_str() + "}) ";
        let finish = String::from("CREATE (lvl) -[:CONTAINS]->(:FIELD {type: 'finish', x: ") +
            finish.x.to_string().as_str() + ", y: " + finish.y.to_string().as_str() + "})";

        let level: String = String::from("CREATE (lvl:LEVEL {index: ") + index.to_string().as_str() +
            ", width: " + dimensions.width.to_string().as_str() + ", height: " + dimensions.height.to_string().as_str() + ", author: '" + username + "'})";

        let query = String::from("WITH ") + field_types.as_str() + x_coords.as_str() + y_coords.as_str() + level.as_str() + 
        " FOREACH (i IN RANGE(0, SIZE(fieldTypes)-1) |
            CREATE (lvl)-[:CONTAINS]->(:FIELD {type: fieldTypes[i], x: Xs[i], y: Ys[i]})
        ) " + start.as_str() + finish.as_str();

        println!("{}", query);

        return match self.run_query(query).await {
            Ok(()) => true,
            Err(_e) => false,
        };

       
    }


    pub async fn add_field(&self, index: i32, x: i32, y: i32, type_: &str) -> bool
    {
        let query = String::from("MATCH (level:LEVEL {index: ") + index.to_string().as_str() + "})
        CREATE (level) -[:CONTAINS]-> (:FIELD {type: \"" + type_ + "\", x: " + x.to_string().as_str() + ", y: " + y.to_string().as_str() + "})";

        //self.run_query(query).await.unwrap();
        match self.run_query(query).await {
            Ok(()) => return true,
            Err(_e) => return false,
        };
    }


    pub async fn get_start(&self, index: i32) ->Result<Option<Field>> {
        let query: String = String::from("match (lvl:LEVEL {index: ") + index.to_string().as_str() + "}) -[:CONTAINS]-> (obs:FIELD{type: \"start\"}) \
            return toString(obs.x) as x, toString(obs.y) as y";

        let mut result = self.query(query).await.unwrap();
        let field: Field;

        let row = match result.next().await {
            Ok(v) => {
                match v {
                    Some(t) => t,
                    None => return Ok(None),
                }
            }
            Err(e) => return Err(e),
        };

        let x: i32 = row.get::<String>("x").unwrap().parse().unwrap();
        let y: i32 = row.get::<String>("y").unwrap().parse().unwrap();

        field = Field{x, y};
        Ok(Some(field))
    }


    pub async fn get_finish(&self, index: i32) ->Result<Option<Field>> {
        let query: String = String::from("match (lvl:LEVEL {index: ") + index.to_string().as_str() + "}) -[:CONTAINS]-> (obs:FIELD{type: \"finish\"}) \
            return toString(obs.x) as x, toString(obs.y) as y";

        let mut result = self.query(query).await.unwrap();
        let field: Field;

        let row = match result.next().await {
            Ok(v) => {
                match v {
                    Some(t) => t,
                    None => return Ok(None),
                }
            }
            Err(e) => return Err(e),
        };

        let x: i32 = row.get::<String>("x").unwrap().parse().unwrap();
        let y: i32 = row.get::<String>("y").unwrap().parse().unwrap();

        field = Field{x, y};
        Ok(Some(field))
    }


    pub async fn get_obstacles(&self, index: i32) ->Result<Vec<Field>> {
        let query: String = String::from("match (lvl:LEVEL {index: ") + index.to_string().as_str() + "}) -[:CONTAINS]-> (obs:FIELD{type: \"obstacle\"}) \
            return toString(obs.x) as x, toString(obs.y) as y";

        let mut result = self.query(query).await.unwrap();


        let mut vec = Vec::<Field>::new();

        while let Ok(Some(row)) = result.next().await {
            let x: i32 = row.get::<String>("x").unwrap().parse().unwrap();
            let y: i32 = row.get::<String>("y").unwrap().parse().unwrap();

            vec.push(Field{x, y});
        }

        Ok(vec)
    }


    pub async fn get_stars(&self, index: i32) ->Result<Vec<Field>> {
        let query: String = String::from("match (lvl:LEVEL {index: ") + index.to_string().as_str() + "}) -[:CONTAINS]-> (obs:FIELD{type: \"star\"}) \
            return toString(obs.x) as x, toString(obs.y) as y";

        let mut result = self.query(query).await.unwrap();


        let mut vec = Vec::<Field>::new();

        while let Ok(Some(row)) = result.next().await {
            let x: i32 = row.get::<String>("x").unwrap().parse().unwrap();
            let y: i32 = row.get::<String>("y").unwrap().parse().unwrap();

            vec.push(Field{x, y});
        }

        Ok(vec)
    }


    pub async fn get_level_dimensions(&self, index: i32) -> Result<LevelDimensions> {
        let query = format!(
            "MATCH (level:LEVEL {{index: {}}}) RETURN toString(level.width) AS width, toString(level.height) AS height",
            index
        );

        let mut result = match self.query(query).await {
            Ok(row) => row,
            Err(e) => return Err(e),
        };

        let mut dimensions = LevelDimensions { width: 0, height: 0 };

        while let Ok(Some(row)) = result.next().await {
            let width = row.get::<String>("width").unwrap().parse().unwrap();
            let height = row.get::<String>("height").unwrap().parse().unwrap();

            dimensions = LevelDimensions{width, height};
        }

        Ok(dimensions)
    }


    pub async fn get_level_fields(&self, index: i32) -> Result<(Field, Field, Vec<Field>, Vec<Field>)> {
        let obstacles = self.get_obstacles(index).await?;
        let stars = self.get_stars(index).await?;
        let start = match self.get_start(index).await {
            Ok(Some(start)) => start,
            Ok(None) => Field{x: -1, y: -1},
            Err(_) => Field{x: -1, y: -1},
        };
        let finish = match self.get_finish(index).await {
            Ok(Some(finish)) => finish,
            Ok(None) => Field{x: -1, y: -1},
            Err(_) => Field{x: -1, y: -1},
        };

        Ok((start, finish, obstacles, stars))
    }


    pub async fn get_levels(&self, username: &str) -> Result<Vec<i32>> {
        let query: String = String::from("MATCH (level:LEVEL) WHERE level.author = 'admin' OR level.author = '") 
            + username + "' RETURN toString(level.index) AS index, toString(level.height)";

        let mut result = self.query(query).await.unwrap();

        let mut vec = Vec::<i32>::new();

        while let Ok(Some(row)) = result.next().await {
            let index: i32 = row.get::<String>("index").unwrap().parse().unwrap();

            vec.push(index);
        }

        vec.sort();

        Ok(vec)
    }


    pub async fn get_available_index(&self) -> Result<i32> {
        let query: String = String::from(
        "MATCH (level:LEVEL)
            WHERE level.index < 10000
            RETURN toString(MAX(level.index)) AS max_index",
        );

        let mut result = self.query(query).await.unwrap();

        if let Ok(Some(row)) = result.next().await {
            let index: i32 = row.get::<String>("max_index").unwrap().parse().unwrap();

            return Ok(index + 1);
        }

        Ok(-1)
    }


    pub async fn return_field_matrix(&self, index: i32) -> Result<(Vec<Vec<i32>>, Field)>
    {
        let mut start = Field{x:0, y:0};
        let query: String = String::from("match (lvl:LEVEL {index: ") + index.to_string().as_str() + "}) -[:CONTAINS]-> (obs:FIELD) \
                    return toString(lvl.width) as width, toString(lvl.height) as height, toString(obs.x) as x, toString(obs.y) as y, obs.type as type";


        let mut result = self.query(query).await.unwrap();

        let row = match result.next().await {
            Ok(v) => {
                match v {
                    Some(t) => t,
                    None => return Ok((Vec::<Vec<i32>>::new(), start)),
                }
            }
            Err(e) => return Err(e),
        };

        let width: usize = row.get::<String>("width").unwrap().parse().unwrap();
        let height: usize =  row.get::<String>("height").unwrap().parse().unwrap();

    
        let mut matrix = vec![vec![0; height]; width];
    
        let x: usize = row.get::<String>("x").unwrap().parse().unwrap();
        let y: usize = row.get::<String>("y").unwrap().parse().unwrap();
        let field_type = row.get::<String>("type").unwrap();
    
        match field_type.as_str() {
            "obstacle" => {
                matrix[x][y] = 1;
            },
            "start" => {
                matrix[x][y] = 2;
                start = Field{x: x as i32, y: y as i32};
            },
            "finish" => {
                matrix[x][y] = 3;
            }
            "star" => {
                matrix[x][y] = 4;
            }
            _ => matrix[x][y] = 0,
        };

        while let Ok(Some(row)) = result.next().await {
            let x: usize = row.get::<String>("x").unwrap().parse().unwrap();
            let y: usize = row.get::<String>("y").unwrap().parse().unwrap();
            let field_type = row.get::<String>("type").unwrap();

            match field_type.as_str() {
                "obstacle" => {
                    matrix[x][y] = 1;
                },
                "start" => {
                    matrix[x][y] = 2;
                    start = Field{x: x as i32, y: y as i32};
                },
                "finish" => {
                    matrix[x][y] = 3;
                }
                "star" => {
                    matrix[x][y] = 4;
                }
                _ => {}
            };
        }

        Ok((matrix, start))
    }


    pub async fn run_query(&self, query: String) -> Result<()>
    {
        let txn = self.graph.start_txn().await.unwrap();
        txn.run_queries(vec![
            neo4rs::query(query.clone().as_str()),
        ])
        .await
        .unwrap();
        txn.commit().await //or txn.rollback().await.unwrap();
    }
}


#[cfg(test)]
mod tests {
    use crate::database::Database;
    use crate::config;
    #[actix_rt::test]
    async fn levels_ok(){
        let db_config = config::Config::default();

        
        let db = Database::new(db_config).await.unwrap();

        let query: String = String::from("MATCH (level:LEVEL) WHERE NOT EXISTS { MATCH (level)-[:CONTAINS]->(level_item:LEVEL_ITEMS {type: \"start\"})} RETURN toString(COUNT(level)) as number");
      
        let mut result = db.query(query).await.unwrap();
       
        let row = result.next().await.unwrap().unwrap();

        let number: usize = row.get::<String>("number").unwrap().parse().unwrap();
    

        assert_eq!(0, number);
    }

    #[actix_rt::test]
    async fn two_same_commands(){
        let db_config = config::Config::default();

        let db = Database::new(db_config).await.unwrap();

        db.register_user("Testovaci_user", "test").await;

        let first = db.add_command("kkk", "pokus", "Testovaci_user").await;

        let second = db.add_command("kkk", "pokus", "Testovaci_user").await;

        let query: String = String::from(" MATCH (u:USER{username: \"Testovaci_user\"}) -[:OWNS] -> (c:COMMAND) detach delete u, c");
        let _u = db.run_query(query).await;

        assert_ne!(first, second);
    }


    #[actix_rt::test]
    async fn delete_existing_commands(){     //should return true however no changes in db will occur
        let db_config = config::Config::default();

        let db = Database::new(db_config).await.unwrap();

        db.register_user("Testovaci_user", "test").await;

        let _first = db.add_command("kkk", "pokus", "Testovaci_user").await;

        let deleted = db.delete_command("Testovaci_user", "pokus").await;

        let query: String = String::from(" MATCH (u:USER{username: \"Testovaci_user\"}) detach delete u");
        let _u = db.run_query(query).await;

        assert_eq!(true, deleted);
    }


    #[actix_rt::test]
    async fn delete_nonexisting_commands(){     //should return true however no changes in db will occur
        let db_config = config::Config::default();

        let db = Database::new(db_config).await.unwrap();

        db.register_user("Testovaci_user", "test").await;

        let first = db.delete_command("Testovaci_user", "pokus").await;


        let query: String = String::from(" MATCH (u:USER{username: \"Testovaci_user\"}) detach delete u");
        let _u = db.run_query(query).await;

        assert_ne!(false, first);
    }
}
