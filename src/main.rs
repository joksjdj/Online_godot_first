use server::{};

use code::{connect_db, ActiveGame, CurrentGame};

use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder, Error, HttpResponse};

use local_ip_address::local_ip;

use sqlx::{MySqlPool, query_as};

use std::fs;

#[get("/page/{number}/")]
async fn page(
    page: web::Path<String>,
    pool: web::Data<MySqlPool>
    ) -> Result<impl Responder, Error> {

    let offset: i64 = page.parse().unwrap();

    let rows = query_as::<_, ActiveGame>(
        "SELECT id, lobby_name, playing FROM active_games LIMIT 5 OFFSET ?"
    )
        .bind((offset - 1) * 5)
        .fetch_all(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("OFFSET: {:?}\nResult: {:?}", (offset - 1) * 5, rows);

    Ok(web::Json(rows))
}

#[get("/lobby/{id}/")]
async fn lobby(
    id: web::Path<i64>,
    pool: web::Data<MySqlPool>
    ) -> impl Responder {
    
    let rows = query_as::<_, CurrentGame>(
        "SELECT * FROM active_games WHERE id = ?"
    )
        .bind(*id)
        .fetch_one(&**pool)
        .await;

    println!("Result: {:?}", rows);

    let html = fs::read_to_string("static/lobby.html")
        .expect("Failed to read HTML file");
    HttpResponse::Ok()
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\n\n");
    
    dotenvy::dotenv().ok();

    let pool = connect_db().await.expect("Failed to connect DB");

    match local_ip() {
        Ok(ip) => println!("Running on: http//{}:8080/", ip),
        Err(e) => println!("Could not get IP: {}", e),
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(page)
            .service(lobby)
            .service(Files::new("/static", "static").show_files_listing())
            .service(Files::new("/", "./static").index_file("index.html"))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}