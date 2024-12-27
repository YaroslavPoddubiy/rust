use warp::{Filter, Rejection, Reply, ws::{Message, WebSocket}};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use sqlx::{sqlite::SqlitePool, Executor, Pool, Sqlite};
use std::sync::{Arc, Mutex};
use futures::{StreamExt, FutureExt};

#[derive(Serialize, Deserialize)]
struct Claims {
    username: String,
    exp: usize,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

#[derive(Deserialize, Serialize)]
struct ChatMessage {
    username: String,
    text: String,
}

type Clients = Arc<Mutex<Vec<tokio::sync::mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

#[tokio::main]
async fn main() {
    let static_files = warp::path("static").and(warp::fs::dir("./static"));

    let db_pool = SqlitePool::connect("sqlite:chat.db").await.unwrap();
    sqlx::migrate!("db/migrations").run(&db_pool).await.unwrap();

    let clients: Clients = Arc::new(Mutex::new(Vec::new()));

    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(handle_login);

    let registration_route = warp::path("registration")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(handle_registration);

    let messages_route = warp::path("messages")
        .and(warp::get())
        .and(with_db(db_pool.clone()))
        .and_then(handle_get_messages);

    let chat_route = warp::path("chat")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and(with_db(db_pool.clone()))
        .map(|ws: warp::ws::Ws, clients, db_pool| {
            ws.on_upgrade(move |socket| handle_connection(socket, clients, db_pool))
        });

    let root_route = warp::path::end()
        .map(|| warp::reply::html(include_str!("../static/index.html")));

    let routes = root_route.or(login_route).or(chat_route).or(messages_route)
        .or(registration_route).or(static_files);

    println!("Server running on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}


async fn handle_login(body: LoginRequest, db_pool: Pool<Sqlite>) -> Result<impl Reply, Rejection> {
    let user = sqlx::query!("SELECT * FROM users WHERE username = ? AND password = ?",
                            body.username, body.password)
        .fetch_optional(&db_pool)
        .await
        .unwrap();

    if let Some(_) = user {
        let claims = Claims {
            username: body.username.clone(),
            exp: 3600,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
            .unwrap();

        Ok(warp::reply::json(&TokenResponse { token }))
    } else {
        Err(warp::reject::custom(CustomRejection("Invalid credentials")))
    }
}

async fn handle_registration(body: LoginRequest, db_pool: Pool<Sqlite>) -> Result<impl Reply, Rejection> {
    let _user = sqlx::query!("SELECT * FROM users WHERE username = ?", body.username)
        .fetch_optional(&db_pool)
        .await
        .unwrap();

    if let Some(_) = _user {
        return Err(warp::reject::custom(CustomRejection("User with the same username already exists")));
    }
    sqlx::query!("INSERT INTO users (username, password) VALUES (?, ?)", body.username, body.password)
        .execute(&db_pool)
        .await
        .unwrap();

    let user = sqlx::query!("SELECT * FROM users WHERE username = ?", body.username)
        .fetch_optional(&db_pool)
        .await
        .unwrap();

    if let Some(_) = user {
        let claims = Claims {
            username: body.username.clone(),
            exp: 3600,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
            .unwrap();
        Ok(warp::reply::json(&TokenResponse { token }))
    } else {
        Err(warp::reject::custom(CustomRejection("Invalid credentials")))
    }

}


async fn handle_get_messages(db_pool: Pool<Sqlite>) -> Result<impl Reply, Rejection> {
    let messages = sqlx::query_as!(
        ChatMessage,
        "SELECT username, text FROM messages"
    )
        .fetch_all(&db_pool)
        .await
        .unwrap();

    Ok(warp::reply::json(&messages))
}


#[derive(Debug)]
struct CustomRejection(&'static str);

impl warp::reject::Reject for CustomRejection {}


async fn handle_connection(ws: WebSocket, clients: Clients, db_pool: Pool<Sqlite>) {
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let rx = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("Error sending message: {}", e);
        }
    }));

    {
        let mut clients_guard = clients.lock().unwrap();
        clients_guard.push(tx.clone());
    }

    while let Some(result) = user_ws_rx.next().await {
        if let Ok(msg) = result {
            if let Ok(text) = msg.to_str() {
                let chat_message: ChatMessage = serde_json::from_str(text).unwrap();

                sqlx::query!(
                    "INSERT INTO messages (username, text) VALUES (?, ?)",
                    chat_message.username,
                    chat_message.text
                )
                    .execute(&db_pool)
                    .await
                    .unwrap();

                let clients_guard = clients.lock().unwrap();
                for client in clients_guard.iter() {
                    if let Err(e) = client.send(Ok(Message::text(text.to_string()))) {
                        eprintln!("Failed to send message: {}", e);
                    }
                }
            }
        }
    }

    {
        let mut clients_guard = clients.lock().unwrap();
        clients_guard.retain(|client| !client.is_closed());
    }
}

fn with_db(
    db_pool: Pool<Sqlite>,
) -> impl Filter<Extract = (Pool<Sqlite>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

fn with_clients(
    clients: Clients,
) -> impl Filter<Extract = (Clients,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
