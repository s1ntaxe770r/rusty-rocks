use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Json, Router};
use rusty_rocks::{establish_connection, get_rocks, insert_rock,models::ErrorResponse};
use serde::Deserialize;




#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(create_rock))
        .route("/rocks", get(rocks));
    println!("booting up server");

    axum::Server::bind(&"0.0.0.0:9093".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct CreateRockRequest {
    name: String,
    kind: String,
}

async fn create_rock(
    Json(payload): Json<CreateRockRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut match establish_connection() {
        Ok(conn) => conn,
        Err(e) => {
            let error_response = ErrorResponse { error: e.to_string() };
            return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response());
        }
    };

    let new_rock = match insert_rock(conn, &payload.name, &payload.kind) {
        Ok(new_rock) => new_rock,
        Err(e) => {
            let error_response = ErrorResponse { error: e.to_string() };
            return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response());
        }
    };

    Ok(Json(new_rock).into_response())
}


async fn rocks() -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut establish_connection().unwrap();
    match get_rocks(conn) {
        Ok(rocks) => Ok(Json(rocks).into_response()),
        Err(e) => {
            let error_response = ErrorResponse { error: e.to_string() };
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response())
        }
    }
}