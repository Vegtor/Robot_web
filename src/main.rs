extern crate neo4rs;
mod config;
mod database;
mod handlers;
mod models;
mod command_parser;
mod defines;

use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};

use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::cookie::{Key, time};

use tera::Tera;
use std::sync::Arc;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_config = config::Config::default();
    let db = Arc::new(database::Database::new(db_config).await.unwrap());

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .app_data(web::Data::new(tera)) // make tera available to all handlers
            .app_data(web::Data::new(db.clone()))
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(), Key::from(&[0; 64])
                )
                .cookie_secure(false)
                .session_lifecycle(PersistentSession::default()
                .session_ttl(time::Duration::minutes(20)))
                .build()
            )
            .wrap(middleware::Logger::default())
            .configure(app_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(Files::new("/styles", "styles").show_files_listing())
            .service(Files::new("/external", "external").show_files_listing())
            .service(Files::new("/js", "js").show_files_listing())
            .service(Files::new("/images", "images").show_files_listing())
            .service(web::resource("/login").route(web::get().to(handlers::login_page)))
            .service(web::resource("/signup").route(web::get().to(handlers::signup_page)))
            .service(web::resource("/editor").route(web::get().to(handlers::editor_page)))
            .service(web::resource("/manual").route(web::get().to(handlers::manual_page)))
            .service(web::resource("/api/login").route(web::post().to(handlers::login)))
            .service(web::resource("/api/signup").route(web::post().to(handlers::signup)))
            .service(web::resource("/api/logout").route(web::post().to(handlers::logout)))
            .service(web::resource("/").route(web::get().to(handlers::index)))
            .service(web::resource("/api/save_command").route(web::post().to(handlers::handle_save)))
            .service(web::resource("/api/parser").route(web::post().to(handlers::parser)))
            .service(web::resource("/api/level").route(web::post().to(handlers::get_level)))
            .service(web::resource("/api/save_level").route(web::post().to(handlers::save_level)))
            .service(web::resource("/stats").route(web::get().to(handlers::stats)))
            .service(web::resource("/commands").route(web::get().to(handlers::commands)))
            .service(web::resource("/api/delete_function").route(web::post().to(handlers::delete_function)))
    );
}
