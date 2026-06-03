use server::{connect_db, Players, LoginRequest, hash_string, check_if_user_exists};

use actix_web::{post, web, App, HttpServer, Responder, Error, HttpResponse};
use actix_cors::Cors;

use local_ip_address::local_ip;

use sqlx::{MySqlPool, query_as};

#[post("/login")]
async fn login(
    credentials: web::Json<LoginRequest>,
    pool: web::Data<MySqlPool>
    ) -> Result<impl Responder, Error> {
    
    let username = &credentials.username;
    let password = hash_string(credentials.password.clone());

    println!("Trying to get players {:?}", username);

    let rows = query_as::<_, Players>(
        "SELECT id, username, highscore, last_game, created_at FROM players WHERE username = ? AND password = ?"
    )
        .bind(username.clone())
        .bind(password.clone())
        .fetch_optional(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("Result: {:?}", rows);

    if rows.is_none() {
        let user_exists = check_if_user_exists(&**pool, username.clone()).await;

        if user_exists == true {
            Ok(HttpResponse::BadRequest().content_type("text/plain").body("Wrong password"))
        } else {
            Ok(HttpResponse::BadRequest().content_type("text/plain").body("user doesnt exist"))
        }
        
    } else {
        Ok(HttpResponse::Ok().json(rows))
    }
}

#[post("/signup")]
async fn signup(
    credentials: web::Json<LoginRequest>,
    pool: web::Data<MySqlPool>
    ) -> Result<impl Responder, Error> {
    
    let username = &credentials.username;
    let password = hash_string(credentials.password.clone());
    
    println!("Trying to get players {:?} {:?}", username, password);

    let insert = match sqlx::query!(
        "INSERT INTO players (username, password)
        VALUES (?, ?)",
        username,
        password
    )
    .execute(&**pool)
    .await
    {
        Ok(res) => res,
        Err(e) => {
            // MySQL duplicate entry error code
            if let Some(db_err) = e.as_database_error() {
                if let Some(mysql_err) = db_err.try_downcast_ref::<sqlx::mysql::MySqlDatabaseError>() {
                    if mysql_err.number() == 1062 {
                        return Ok(
                            HttpResponse::BadRequest()
                                .content_type("text/plain")
                                .body("user already exists")
                        );
                    }
                }
            }
            return Ok(
                HttpResponse::InternalServerError()
                    .content_type("text/plain")
                    .body("database error")
            );
        }
    };
    let rows = query_as::<_, Players>(
        "SELECT id, username, highscore, last_game FROM players WHERE id = ?"
    )
        .bind(insert.last_insert_id())
        .fetch_optional(&**pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("Result: {:?}", rows);
    return Ok(HttpResponse::Ok().json(rows))
    
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
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .service(login)
            .service(signup)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}