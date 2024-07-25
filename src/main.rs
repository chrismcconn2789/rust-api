use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post, delete},
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

async fn delete_todo(id: u64) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!("Todo deleted with id: {}", id)))
        .unwrap()
}

async fn delete_todo_handler(
    axum::extract::Path(id): axum::extract::Path<u64>,
) -> impl IntoResponse {
    delete_todo(id).await
}

async fn get_todo(id: u64) -> Json<Todo> {
    let todo = Todo {
        id,
        name: "Task 1".to_string(),
        complete: false,

    };

    Json(todo)
}

async fn get_todo_handler(
    axum::extract::Path(id): axum::extract::Path<u64>,
) -> impl IntoResponse {
    get_todo(id).await
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
        .route("/todos", get(list_todos))
        .route("/todos/:id", get(get_todo_handler))
        .route("/todos/:id", delete(delete_todo_handler));

    println!("Running on http://localhost:3000");
    
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}