use server::{connect_db, Players, LoginRequest};

use actix_web::{post, web, App, HttpServer, Responder, Error, HttpResponse};

use local_ip_address::local_ip;

use sqlx::{MySqlPool, query_as};

#[post("/login")]
async fn login(
    credentials: web::Json<LoginRequest>,
    pool: web::Data<MySqlPool>
    ) -> Result<impl Responder, Error> {
    
    let username = &credentials.username;
    let password = &credentials.password;
    
    println!("Trying to get players {:?} {:?}", username, password);

    let rows = query_as::<_, Players>(
        "SELECT id, username, highscore, last_game FROM players WHERE username = ? AND password = ?"
    )
        .bind(username.clone())
        .bind(password.clone())
        .fetch_one(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("Result: {:?}", rows);

    Ok(web::Json(rows))
}

#[post("/signin")]
async fn signin(
    credentials: web::Json<LoginRequest>,
    pool: web::Data<MySqlPool>
    ) -> Result<impl Responder, Error> {
    
    let username = &credentials.username;
    let password = &credentials.password;
    
    println!("Trying to get players {:?} {:?}", username, password);

    sqlx::query(
        "INSERT INTO players (username, password)
        VALUES (?, ?)"
    )
        .bind(username.clone())
        .bind(password.clone())
        .execute(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
        
    let rows = query_as::<_, Players>(
        "SELECT id, username, highscore, last_game FROM players WHERE username = ? AND password = ?"
    )
        .bind(username.clone())
        .bind(password.clone())
        .fetch_one(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("Result: {:?}", rows);

    Ok(web::Json(rows))
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
            .service(login)
            .service(signin)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}