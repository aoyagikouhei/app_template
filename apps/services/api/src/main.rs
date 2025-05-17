use axum::{
    http::StatusCode, routing::{get, post}, Extension, Json, Router
};
use common::config::Config;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[tokio::main]
async fn main() {
    // initialize tracing
    logs::setup_tracing("api");
    let config = Config::setup();
    let pg_pool = config.pg_pool().await.unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .layer(Extension(pg_pool));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(FromRow)]
struct MyType{
    sum: i32,
}

// basic handler that responds with a static string
async fn root(Extension(pg_pool): Extension<PgPool>,) -> String {
    let res: MyType = sqlx::query_as("SELECT 1 + 1 AS sum")
        .fetch_one(&pg_pool)
        .await
        .unwrap();
    format!("Hello, World! The sum is: {}", res.sum)
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}