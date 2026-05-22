use sqlx::{MySqlPool, mysql::MySqlPoolOptions, FromRow};
use sqlx::types::chrono::{DateTime, Utc};

use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::{Instant, Duration};
use std::fs;

use once_cell::sync::Lazy;

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio::sync::RwLock;

use futures_util::{StreamExt, SinkExt};

use serde::{Serialize, Deserialize};
use serde_json::json;
use serde_json::Value as JSON_Value;

use glam::{Vec3, Quat};

pub async fn connect_db() -> Result<MySqlPool, sqlx::Error> {
    let start = Instant::now();
    println!("connecting to db...");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    println!(".env: {}", database_url);
    println!("Connected");

    let duration = start.elapsed();
    let ms = (duration.as_secs_f64() * 1000.0).ceil() / 1000.0;
    println!("Time elapsed: {:.3} ms\n", ms);

    Ok(pool)
    
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Players {
    pub id: i64,
    username: String,
    created_at: DateTime<Utc>,
    highscore: i64,
    last_game: i64,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub fn hash_string(unhashed_password: String) -> String {

    if unhashed_password.is_empty() || unhashed_password.chars().any(|c| c.is_whitespace()) {
        return "".to_string()
    }

    let mut hasher = DefaultHasher::new();
    unhashed_password.hash(&mut hasher);

    let password = hasher.finish().to_string();

    password
}

pub async fn check_if_user_exists(pool: &MySqlPool, username: String) -> bool {
    let find_user = sqlx::query(
            "SELECT username FROM players WHERE username = ?"
        )
            .bind(username.clone())
            .fetch_optional(pool)
            .await
            .expect("DB error");

    let user_exists: bool;
    if find_user.is_none() {
        user_exists = false;
    } else {
        user_exists = true;
    }

    return user_exists
}

pub static MAP_JSON: Lazy<RwLock<JSON_Value>> = Lazy::new(|| {
    let data = fs::read_to_string("static/map_collision.json").unwrap();

    let json: JSON_Value = serde_json::from_str(&data).unwrap();

    RwLock::new(json)
});

pub static GLOBAL_JSON: Lazy<RwLock<JSON_Value>> = Lazy::new(|| {
    RwLock::new(serde_json::json!({}))
});

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlayerState {
    #[serde(default = "default_health")]
    pub health: i32,

    #[serde(default = "default_grapple")]
    pub grapple_cooldown: f32,

    #[serde(default)]
    pub score: i32,

    #[serde(default = "default_vec3")]
    pub pos: Vec3,

    #[serde(default = "default_quat")]
    pub rot: Quat,

    #[serde(default = "default_vec3")]
    pub velocity: Vec3,

    #[serde(default = "default_is_on_floor")]
    pub is_on_floor: bool,
}

fn default_health() -> i32 { 3 }
fn default_grapple() -> f32 { 0.0 }
fn default_vec3() -> Vec3 { Vec3::ZERO }
fn default_quat() -> Quat { Quat::IDENTITY }
fn default_is_on_floor() -> bool { false }

fn vec3_for_math(unusable_vec3: &JSON_Value) -> Vec3 {
    let vec_arr = unusable_vec3.as_array().unwrap();
    let vec = Vec3::new(
        vec_arr[0].as_f64().unwrap() as f32,
        vec_arr[1].as_f64().unwrap() as f32,
        vec_arr[2].as_f64().unwrap() as f32,
    );

    return vec
}

const TICK_RATE: u64 = 60;
const TICK_TIME: Duration = Duration::from_nanos(1_000_000_000 / TICK_RATE);

pub async fn run_game_loop(lobby_id: String) {
    let mut last_tick = Instant::now();

    loop {
        let now = Instant::now();
        if now - last_tick >= TICK_TIME {
            let dt = (now - last_tick).as_secs_f32();
            last_tick = now;

            let lobby = {
                let json = GLOBAL_JSON.read().await;
                json[&lobby_id].clone()
            };
            let mut lobby = lobby;

            lobby["global_cooldown"] = json!(0.0);

            // gravity vector (same as Godot: (0, -9.8, 0))
            let gravity = Vec3::new(0.0, -9.8, 0.0);

            if let Some(arr) = lobby["players"].as_object_mut() {
                for player in arr.values_mut() {

                    let map = {
                        let json = MAP_JSON.read().await;
                        json.as_array().unwrap().clone()
                    };
                    for obj in map {
                        println!("{:?}", obj["faces"])
                    }

                    if player["is_on_floor"] == false {
                        let usable_vel = vec3_for_math(&player["velocity"]);

                        let vel = usable_vel + gravity * 2.0 * dt;
                        player["velocity"] = json!(vel);

                        // apply velocity to position
                        let pos = vec3_for_math(&player["pos"]);
                        player["pos"] = json!(pos + vel * dt);
                    }

                }
                {
                    let mut json = GLOBAL_JSON.write().await;
                    json[&lobby_id] = lobby
                };
            }
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
}

pub async fn start_ws_server() {
    let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    println!("WebSocket server running on ws://0.0.0.0:8081");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            println!("Client connected");

            let (mut write, mut read) = ws_stream.split();

            while let Some(Ok(msg)) = read.next().await {
                if msg.is_text() {
                    let text = msg.to_text().unwrap();
                    let message: serde_json::Value = serde_json::from_str(text).unwrap();

                    println!("{:?}", message);

                    // Extract fields safely
                    let id = message["id"].as_str().map(|s| s.to_string());
                    let req = message["req"].as_str().map(|s| s.to_string());
                    let lobby_id = message["lobby_id"].as_str().map(|s| s.to_string());

                    // If missing fields → ignore
                    if id.is_none() || req.is_none() || lobby_id.is_none() {
                        continue;
                    }

                    let id = id.unwrap();
                    let req = req.unwrap();
                    let lobby_id = lobby_id.unwrap();

                    println!("{:?} {:?} {:?}", id, req, lobby_id);

                    // Handle request
                    if req == "create_lobby" {
                        let mut json = GLOBAL_JSON.write().await;
                        json[&lobby_id] = json!({
                            "global_cooldown": 0.0,
                            "enemies_left": 7,
                        });
                        json[&lobby_id]["players"] = json!({});
                        json[&lobby_id]["players"][&id] = json!(PlayerState::default());
                    }

                    if req == "join_lobby" {
                        let mut json = GLOBAL_JSON.write().await;
                        json[&lobby_id]["players"][&id] = json!(PlayerState::default());
                    }

                    if req == "play" {
                        let move_id = lobby_id.clone();
                        tokio::spawn(async move {
                            run_game_loop(move_id).await;
                        });

                    }
                    
                    if req == "get_info" {
                        let value = {
                            let json = GLOBAL_JSON.read().await;
                            json[&lobby_id].clone()
                        };
                        write.send(value.to_string().into()).await.unwrap();
                    }
                }
            }

            println!("Client disconnected");
        });
    }
}
