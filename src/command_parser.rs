//use std::io::Error;
use std::io::ErrorKind;
use crate::models::Field;
use crate::defines::*;
pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub heading: i32,
}

const RAYS: &'static [[Field; 6]; 2] = &[
    [
        Field { x: 0, y: 1 },
        Field { x: -1, y: 0 },
        Field { x: -1, y: -1 },
        Field { x: 0, y: -1 },
        Field { x: 1, y: -1 },
        Field { x: 1, y: 0 },
    ],
    [
        Field { x: 0, y: 1 },
        Field { x: -1, y: 1 },
        Field { x: -1, y: 0 },
        Field { x: 0, y: -1 },
        Field { x: 1, y: 0 },
        Field { x: 1, y: 1 },
    ],
];



pub fn parse_command(command_to_parse: String, map: &Vec<Vec<i32>>, start: Field, max_num_of_steps: i32) -> Result<(Vec<char>, RobotState, i32), std::io::Error> {
    let mut robot = Robot{x: start.x, y: start.y, heading: 0};
    let command: Vec<char> = command_to_parse.chars().collect();    //I need to work with index operator []
    let mut steps = 0;
    let mut stars: i32 = 0;
    parse_exec(command, map, &mut robot, &mut steps, max_num_of_steps, &mut stars)
}


pub fn parse_exec(command: Vec<char>, map: &Vec<Vec<i32>>, robot: &mut Robot, steps: &mut i32, max_num_of_steps: i32, stars: &mut i32) -> Result<(Vec<char>, RobotState, i32), std::io::Error> {
    let length = command.len();

    let num_cols = map.len() as i32;
    let num_rows = if let Some(first_row) = map.first() {
        first_row.len() as i32
    } else {
        0
    };
    //println!("{} {}", num_rows, num_cols);

    let mut result = Vec::<char>::new();                //Vector of chars which will be returned

    let mut i = 0;      //Variable for iteration
    while i < length {
        match command[i] {
            'c' => { //If the char is c
                i += 2;
                let cycles = match number_of_cycles(command[i..].to_vec()) {
                    Ok((cycles, index)) => {
                        i += index + 1;
                        cycles
                    },
                    Err(_) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                };

                let sub_string = match sub_body(command[i..].to_vec()) {     //It will find the body of loop,
                    Ok(vec) => vec,                                                           //for example if command is c(4){kr}kr, it returns {kr}
                    Err(_e) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                };


                i += sub_string.len() + 1;  //We have to move i forward behind the loop body
                for _ in 0..cycles {    //We have to append the parsed body of the loop so many times as is the number of loops
                    let _state = match parse_exec(sub_string.clone(), map, robot, steps, max_num_of_steps, stars){
                        Ok((vec, st, sub_steps)) => {
                            //println!("{:?}", vec);
                            result.extend(vec);
                            match st  {
                                RobotState::Finished => return Ok((result, st, *steps)),
                                RobotState::Crashed => return Ok((result, st, *steps)),
                                RobotState::OutOfFuel => return Ok((result, st, *steps)),
                                _ => {},
                            };
                        },
                        Err(_e) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                    };
                }
            },
            'p' => {     //If the char is p
                i += 2 ;
                //println!("command = {:?}", command[i..length].to_vec());
                let condition = match parse_condition(command[i..].to_vec()) {
                    Ok((con, index)) => {
                        i += index + 1;
                        match con {
                            's' => 3,
                            'j' => 0,
                            'z' => 6,
                            'q' => 2,
                            'e' => 4,
                            'y' => 1,
                            'v' => 5,
                            _ => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                        }
                       
                        //con
                    },
                    Err(_e) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                };   


                let condition_accepted = match_condition(condition, map, robot);

                let sub_string = match sub_body(command[i..length].to_vec()) {     //Similar to c
                    Ok(vec) => vec,
                    Err(_e) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                };
                /*let (body, _state) = match parse_exec(sub_string.clone(), &map,robot){   //We call recursively parser on the body found on the line above
                    Ok((vec, st)) => (vec, st),
                    Err(_e) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                };*/
                i += sub_string.len() + 1;  //We move i forward but there is no number behind the p

                if condition_accepted {
                    let (body, state, _sub_steps) = match parse_exec(sub_string.clone(), &map,robot, steps, max_num_of_steps, stars){   //We call recursively parser on the body found on the line above
                        Ok((vec, st, sub_steps)) => (vec, st, *steps),
                        Err(_e) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                    };
                    result.extend(body.clone());   //We append it just once

                    match state  {
                        RobotState::Finished => return Ok((result, state, *steps)),
                        RobotState::Crashed => return Ok((result, state, *steps)),
                        RobotState::OutOfFuel => return Ok((result, state, *steps)),
                        _ => {},
                    };
                }
                
            },
            'n' => {
                i += 2 ;
                //println!("command = {:?}", command[i..length].to_vec());
                let condition = match parse_condition(command[i..].to_vec()) {
                    Ok((con, index)) => {
                        i += index + 1;
                        match con {
                            's' => 3,
                            'j' => 0,
                            'z' => 6,
                            'q' => 2,
                            'e' => 4,
                            'y' => 1,
                            'v' => 5,
                            _ => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                        }
                       
                        //con
                    },
                    Err(_e) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                };   


                let condition_accepted = !match_condition(condition, map, robot);

                let sub_string = match sub_body(command[i..length].to_vec()) {     //Similar to c
                    Ok(vec) => vec,
                    Err(_e) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                };

                i += sub_string.len() + 1;  //We move i forward but there is no number behind the p

                if condition_accepted {
                    let (body, state, _substeps) = match parse_exec(sub_string.clone(), &map,robot, steps, max_num_of_steps, stars){   //We call recursively parser on the body found on the line above
                        Ok((vec, st, _sub_steps)) => {(vec, st, *steps)},                        
                        Err(_e) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                    };
                    result.extend(body.clone());   //We append it just once

                    match state  {
                        RobotState::Finished => return Ok((result, state, *steps)),
                        RobotState::Crashed => return Ok((result, state, *steps)),
                        RobotState::OutOfFuel => return Ok((result, state, *steps)),
                        _ => {},
                    };
                }
            },
            'd' => {
                i += 2 ;
                //println!("command = {:?}", command[i..length].to_vec());
                let condition = match parse_condition(command[i..].to_vec()) {
                    Ok((con, index)) => {
                        i += index + 1;
                        match con {
                            's' => 3,
                            'j' => 0,
                            'z' => 6,
                            'q' => 2,
                            'e' => 4,
                            'y' => 1,
                            'v' => 5,
                            _ => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                        }
                       
                        //con
                    },
                    Err(_e) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                };   

                let sub_string = match sub_body(command[i..length].to_vec()) {     //Similar to c
                    Ok(vec) => vec,
                    Err(_e) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                };
             
                i += sub_string.len() + 1;

                //let condition_accepted = match_condition(condition, map, robot);

                while !match_condition(condition, map, robot) {
                    let (body, state, _substeps) = match parse_exec(sub_string.clone(), &map,robot, steps, max_num_of_steps, stars){   //We call recursively parser on the body found on the line above
                        Ok((vec, st, _sub_steps)) => (vec, st, *steps),
                        Err(_e) => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
                    };
                    result.extend(body.clone());   //We append it just once
                    match state  {
                        RobotState::Finished => return Ok((result, state, *steps)),
                        RobotState::Crashed => return Ok((result, state, *steps)),
                        RobotState::OutOfFuel => return Ok((result, state, *steps)),
                        _ => {},
                    };
                }


            },
            'k' => {
                i += 1;
                let ray: Field = RAYS[(robot.x as usize) % 2][robot.heading as usize];
                let new_x = robot.x + ray.x;
                let new_y = robot.y + ray.y;

                //println!("{} {}", new_x, new_y);

                if new_x < 0 || new_y < 0 || new_x >= num_cols || new_y >= num_rows {
                    return Ok((result, RobotState::Crashed, *steps));
                }

                if map[new_x as usize][new_y as usize] == 1 {     //If robot should move into the obstacle, we end
                    return Ok((result, RobotState::Crashed, *steps));
                }

                if map[new_x as usize][new_y as usize] == 3 {     //If robot should move into the finish, we end
                    result.push('k');
                    *steps += 1;
                    return Ok((result, RobotState::Finished, *steps));
                }
                
                if map[new_x as usize][new_y as usize] == 4 {     
                    *stars += 1;
                    *steps -= 3;
                    println!("Star detected {}", steps);
                    //return Ok((result, RobotState::Finished));
                }

                robot.x = new_x;
                robot.y = new_y;
                result.push('k');

                *steps += 1;

                println!("{}", steps);

                if *steps >= max_num_of_steps {
                    return Ok((result, RobotState::OutOfFuel, *steps));
                }

                
            },
            'r' => {
                robot.heading = (robot.heading + 1) % 6;
                result.push(command[i]);
                i += 1;

                *steps += 1;

                println!("{}", steps);

                if *steps >= max_num_of_steps {
                    return Ok((result, RobotState::OutOfFuel, *steps));
                }

            },
            '{' | '}' => i += 1,
            _ => return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!")),
        }
    }

    Ok((result, RobotState::Moving, *steps))
}


fn number_of_cycles(command: Vec<char>) -> Result<(u32, usize), std::io::Error> {
    let length = command.len();
    let mut result= 0;

    let mut i = 0;

    while i < length {
        if command[i] == ')' {
            break;
        }

        if !command[i].is_digit(10) {
            i = length;
            break;
        }

        let digit = command[i].to_digit(10).unwrap();
        result = result * 10 + digit;
        
        i += 1;
    }
    if i == length {
        return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!"));
    }
    
    return Ok((result, i))
}


fn parse_condition(command: Vec<char>) -> Result<(char, usize), std::io::Error> {
    let length = command.len();
    let mut result: char = 'w';

    let mut i = 0;

    while i < length {
        if command[i] == ')' {
            break;
        }

        result = command[i];

        i += 1;

        if i > 1 {
            i = length;
        }
    }
    if i == length {
        return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!"));
    }
    
    return Ok((result, i))
}


fn sub_body(command: Vec<char>) -> Result<Vec<char>, std::io::Error> {
    let length = command.len();

    if length == 0 || command[0] != '{' {
        return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!"));
    }

    let mut result = Vec::<char>::new();

    let mut i = 1;

    let mut left = 1;
    let mut right = 0;
    while i < length {  //Until we iterate through all vector
        if command[i] == '{' {
            left += 1;
        }
        else if command[i] == '}' {
            right += 1;
        }

        if left == right {  //If the number of right brackets is the same as the number of left brackets, we are at the end of the body
            break;
        }

        result.push(command[i]);
        i += 1;
    }
    if i == length {
        return Err(std::io::Error::new(ErrorKind::InvalidInput,"oh no!"));
    }

    Ok(result)
}


fn match_condition(condition: i32, map: &Vec<Vec<i32>>, robot: &mut Robot) -> bool {
    if condition != 6 { //6 means obstacle
        return condition == robot.heading;
    }

    let num_cols = map.len() as i32;
    let num_rows = if let Some(first_row) = map.first() {
        first_row.len() as i32
    } else {
        0
    };

    let ray: Field = RAYS[(robot.x as usize) % 2][robot.heading as usize];
    let new_x = robot.x + ray.x;
    let new_y = robot.y + ray.y;


    if new_x < 0 || new_y < 0 || new_x >= num_cols || new_y >= num_rows {
        return true;
    }
    if map[new_x as usize][new_y as usize] == 1 {     //If robot should move into the obstacle, we end
        return true;
    }
    false
}