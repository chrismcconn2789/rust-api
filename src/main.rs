use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct Todo {
    id: u64,
    name: String,
    complete: bool,
}

async fn create_todo() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("Todo created successfully"))
        .unwrap()
}

async fn list_todos() -> Json<Vec<Todo>> {
    let todos = vec![
        Todo {
            id: 1,
            name: "Task 1".to_string(),
            complete: false,
        },
        Todo {
            id: 2,
            name: "Task 2".to_string(),
            complete: true,
        },
    ];
    Json(todos)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Health check: API is running" }))
        .route("/todos", post(create_todo))
        .route("/todos", get(list_todos));

    println!("Running on http://localhost:3000");
    
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}