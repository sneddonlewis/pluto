use std::env;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::Serialize;
use tokio_postgres::{NoTls, Error};

#[derive(Debug, Serialize)]
struct Technology {
    name: String,
    details: String,
}

async fn get_technologies() -> Result<Vec<Technology>, Error> {
    let db_host = env::var("POSTGRES_HOST").unwrap_or(String::from("localhost"));
    let db_username = env::var("POSTGRES_USER").unwrap_or(String::from("goxygen"));
    let db_password = env::var("POSTGRES_PASSWORD").unwrap_or(String::from("pass"));
    let db_name = env::var("POSTGRES_DB").unwrap_or(String::from("goxygen"));
    let db_url = format!(
        "postgresql://{}:{}@{}/{}",
        db_username,
        db_password,
        db_host,
        db_name,
    );

    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let stmt = client.prepare("SELECT name, details FROM technologies").await?;
    println!("statement ran");

    let rows = client.query(&stmt, &[]).await?;

    let mut technologies = Vec::new();

    for row in rows {
        let name: String = row.get(0);
        let details: String = row.get(1);

        let technology = Technology { name, details };
        technologies.push(technology);
    }

    Ok(technologies)
}

#[get("/api/technologies")]
async fn get_technologies_handler() -> impl Responder {
    match get_technologies().await {
        Ok(technologies) => HttpResponse::Ok().json(technologies),
        Err(err) => {
            eprintln!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors_policy = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors_policy)
            .service(get_technologies_handler)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}