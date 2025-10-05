use actix_web::{web, HttpResponse, HttpRequest, Result};
use crate::models::{Command, CommandToParse, HandlerSaveResponse, LoginParams, SignupParams, SignupResponse, ParserResponse, Level, LevelMap, CommandDeleteRequest};
use actix_session::Session;
use crate::database::Database;
use crate::command_parser::parse_command;
use crate::defines::*;
use tera::{Tera, Context};


pub async fn index(db: web::Data<std::sync::Arc<Database>>, tera: web::Data<Tera>, session: Session) -> Result<HttpResponse> {
    let redirect_to_login = || {
        Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/login"))
            .finish())
    };

    let username = match session.get::<String>("username")? {
        Some(username) => username,
        None => return redirect_to_login(),
    };

    let user_commands = match db.get_user_commands(username.as_str()).await {
        Ok(user_commands) => user_commands,
        Err(_) => return redirect_to_login(),
    };

    let (start, finish, obstacles, stars) = match db.get_level_fields(1).await {
        Ok(fields) => fields,
        Err(_) => return redirect_to_login(),
    };

    let dimensions = match db.get_level_dimensions(1).await {
        Ok(dim) => dim,
        Err(_) => return redirect_to_login(),
    };

    let levels = match db.get_levels(username.clone().as_str()).await {
        Ok(lvls) => lvls,
        Err(_) => return redirect_to_login(),
    };

    let mut context = Context::new();
    context.insert("functions", &user_commands);
    context.insert("start", &start);
    context.insert("finish", &finish);
    context.insert("obstacles", &obstacles);
    context.insert("stars", &stars);
    context.insert("dimensions", &dimensions);
    context.insert("levels", &levels);

    let res = tera
        .render("main.html", &context)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(res))
}


/// Request and POST Params
pub async fn handle_save(db: web::Data<std::sync::Arc<Database>>, _req: HttpRequest, data: web::Json<Command>, session: Session) -> Result<HttpResponse> { //Handles the save button on main page
    let mut response = HandlerSaveResponse{
        command_saved: false,
        message: "Nepovedlo se uložit příkaz do databáze, pravděpodobně nejste přihlášen".to_string(),
        alert_icon_type: "warning".to_string(), alert_title: "Error".to_string()
    };

    if let Some(username) = session.get::<String>("username")? {    //Checks if the session is registrated

            match db.add_command(data.command.as_str(), data.name.as_str(), username.as_str()).await {  //matches if the command was saved or not
                true => {
                    response.command_saved = true;
                    response.message = "Vaše funkce byla uložena do databáze".to_string();
                    response.alert_icon_type = "success".to_string();
                    response.alert_title = "Úspěšně uloženo do databáze".to_string();
                },
                false => {
                    response.command_saved = false;
                    response.message = "Vaše funkce nebyla uložena do databáze, protože funkce se stejným názvem již existuje".to_string();
                    response.alert_icon_type = "error".to_string();
                    response.alert_title = "Chyba".to_string();
                },
            };

    }

    Ok(HttpResponse::Ok().json(response))
}


pub async fn login_page(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let res = tera
        .render("login.html", &tera::Context::new())
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(res))
}


pub async fn signup_page(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let res = tera
        .render("signup.html", &tera::Context::new())
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(res))
}


pub async fn editor_page(
    tera: web::Data<Tera>,
    session: Session
) -> Result<HttpResponse> {
    if session.get::<String>("username")? == None{
        return Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/login"))
            .finish());
    }

    let res = tera
        .render("editor.html", &tera::Context::new())
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(res))
}


pub async fn manual_page(tera: web::Data<Tera>, session: Session) -> Result<HttpResponse> {
    let res = tera
        .render("manual.html", &tera::Context::new())
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(res))
}


pub async fn login(db: web::Data<std::sync::Arc<Database>>,params: web::Form<LoginParams>, session: Session) -> Result<HttpResponse> {  //logs the user if correct personal data are entered

    let username = params.username.clone();
    let password = params.password.clone();


    match db.login_user(username.as_str(), password.as_str()).await {   //checks whether the user entered correct password
        true => {
            session.insert("username", username)?;

            return Ok(HttpResponse::SeeOther()
                        .append_header(("Location", "/"))
                        .finish());
        },
        false => {
            return Ok(HttpResponse::SeeOther()
                        .append_header(("Location", "/login"))
                        .finish());
        },
    };
}


pub async fn logout(session: Session) -> Result<HttpResponse> {
    session.clear();

    return Ok(HttpResponse::Ok().json("/login"))
}


pub async fn signup(db: web::Data<std::sync::Arc<Database>>, params: web::Json<SignupParams>, _session: Session) -> Result<HttpResponse> {
    //let db_config = config::Config::default();
    //let db = Database::new(db_config).await.unwrap();

    let username = params.username.clone();
    let password = params.password.clone();
    let confirm_password = params.confirm_password.clone();

    if password != confirm_password {
        return Ok(HttpResponse::Ok().json(SignupResponse{signed_up: false, message: "Nebyla zadána stejná hesla".to_string(), title: "Neshodující se hesla".to_string(), icon: "error".to_string()}));
    }

    return match db.register_user(username.as_str(), password.as_str()).await {
        true => {
            Ok(HttpResponse::Ok().json(SignupResponse { signed_up: true, message: "Registrace proběhla úspěšně".to_string(), title: "Úspěšná registrace".to_string(), icon: "success".to_string() }))
        },
        false => {
            Ok(HttpResponse::Ok().json(SignupResponse { signed_up: false, message: "Nepovedlo se provést registraci. Zkuste to, prosím, později.".to_string(), title: "Neúspěšná registrace".to_string(), icon: "error".to_string() }))
        }
    }
}


pub async fn parser(db: web::Data<std::sync::Arc<Database>>, params: web::Json<CommandToParse>, session: Session) -> Result<HttpResponse> {

    let (map, start) = match db.return_field_matrix(params.level.clone()).await {
        Ok((vec, start)) => (vec, start),
        Err(error) => {
            eprintln!("{}", error);
            return Ok(HttpResponse::Ok().json(ParserResponse{error: true, command: params.command.clone(), state: "None".to_string(), num_of_steps: 0}))
        },
    };

    if map.is_empty() {
        return Ok(HttpResponse::Ok().json(ParserResponse{error: true, command: "Tento level neexistuje".to_string(), state: "None".to_string(), num_of_steps: 0}));
    }

    let max_num_of_steps = map.len() * map[0].len();

    let (parsed_command, state, num_of_steps) = match parse_command(params.command.clone(), &map, start, max_num_of_steps as i32){
        Ok((vec, st, steps)) => (String::from_iter(vec),st, steps),
        Err(_e) => ("Intern error".to_string(),RobotState::Crashed, 0)
    };

    
    match state {
        RobotState::Finished => {
            if let Some(username) = session.get::<String>("username")? {
                //db.add_solved_level(params.level.clone(), parsed_command.clone().len() as i32, username.as_str()).await;
                db.add_solved_level(params.level.clone(), num_of_steps, username.as_str()).await;
            }
        },
        _ => {}
    };

    Ok(HttpResponse::Ok().json(ParserResponse{error: false, command: parsed_command.clone(), state: state.to_string(), num_of_steps: num_of_steps}))
}


pub async fn get_level(db: web::Data<std::sync::Arc<Database>>, params: web::Json<Level>, _session: Session) -> Result<HttpResponse> {
    let level : i32 = params.level;

    let dimensions = match db.get_level_dimensions(level).await {
        Ok(dim) => dim,
        Err(_) => return Ok(HttpResponse::Ok().json({})),
    };

    let (start, finish, obstacles, stars) = match db.get_level_fields(level).await {
        Ok(fields) => fields,
        Err(_) => return Ok(HttpResponse::Ok().json({})),
    };

    let response = LevelMap { dimensions, start, finish, obstacles, stars };

    Ok(HttpResponse::Ok().json(response))
}


pub async fn save_level(db: web::Data<std::sync::Arc<Database>>, params: web::Json<LevelMap>, session: Session) -> Result<HttpResponse> {

    if let Some(username) = session.get::<String>("username")? {
        let index = match db.get_available_index().await {
            Ok(i) => i,
            Err(_e) => {return Ok(HttpResponse::Ok().json(false));}
        };
    
        return match db.add_level_with_fields(username.as_str(), index, params.dimensions, params.start, params.finish, params.obstacles.clone(), params.stars.clone(),).await {
            true => Ok(HttpResponse::Ok().json(true)),
            false => Ok(HttpResponse::Ok().json(false))
        };
    }
    else {
        Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/login"))
            .finish())
    }
}


pub async fn stats(db: web::Data<std::sync::Arc<Database>>, tera: web::Data<Tera>, session: Session) -> Result<HttpResponse> {
    if let Some(username) = session.get::<String>("username")? {

        let user_stats = match db.get_user_best_stats(username.as_str()).await {
            Ok(stats) => stats,
            Err(_e) => {
                return  Ok(HttpResponse::SeeOther()
                            .append_header(("Location", "/"))
                            .finish())
            }
        };

        let best_stats = match db.find_best().await {
            Ok(stats) => stats,
            Err(_e) => {
                return  Ok(HttpResponse::SeeOther()
                            .append_header(("Location", "/"))
                            .finish())
            }
        };

        let mut context = Context::new();
        context.insert("user_stats", &user_stats);
        context.insert("best_stats", &best_stats);
        context.insert("user", &username);
      
        let res = tera
            .render("stats.html", &context)
            .map_err(actix_web::error::ErrorInternalServerError)?;

        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(res));
    } else {
        Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/login"))
            .finish())
    }
}

pub async fn commands(db: web::Data<std::sync::Arc<Database>>, tera: web::Data<Tera>, session: Session) -> Result<HttpResponse> {
    if let Some(username) = session.get::<String>("username")? {

        let commands = match db.get_user_commands(username.as_str()).await {
            Ok(commands) => commands,
            Err(_e) => {
                return  Ok(HttpResponse::SeeOther()
                            .append_header(("Location", "/"))
                            .finish())
            }
        };

        let mut context = Context::new();
        context.insert("commands", &commands);

        let res = tera
                            .render("command_list.html", &context)
                            .map_err(actix_web::error::ErrorInternalServerError)?;

        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(res));
    } else {
        Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/login"))
            .finish())
    }
}

pub async fn delete_function(
    db: web::Data<std::sync::Arc<Database>>,
    params: web::Json<CommandDeleteRequest>,
    session: Session,
) -> Result<HttpResponse> {
    if let Some(username) = session.get::<String>("username")? {
        db.delete_command(username.as_str(), params.command.as_str()).await;

        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/login"))
            .finish())
    }
}
