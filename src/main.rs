use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod db;
mod errors;
mod handlers;
mod models;
mod seed;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    print!("Starting server on {}:{}", host, port);

    let run_seeder = std::env::var("RUN_SEEDER")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);

    let db_conn = db::establish_connection()
        .await
        .expect("Failed to connect to database");

    if run_seeder {
        if let Err(e) = seed::seed_database(&db_conn).await {
            eprintln!("Error running seeder: {}", e);
        }
    }

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type", "Authorization"]);

        App::new()
            .wrap(cors)
            .route("/hi", web::get().to(|| async { "hi!" }))
            .service(handlers::user::login)
            .service(handlers::cluster::get_clusters)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
