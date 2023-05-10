use crate::lib::{self, handlers::Database, http};
use actix_web::{web, HttpRequest, HttpResponse};

// The add_user_to_whitelist() endpoint is used to add an user to the provided class_id's
// whitelist. Anyone within this whitelist can access the provided class_id. The whitelist
// feature only works if the user has enabled the class whitelist setting.
#[actix_web::put("/class/{class_id}/whitelist")]
async fn add_user_to_whitelist(
    req: HttpRequest,
    db: web::Data<Database>,
    body: web::Bytes,
) -> HttpResponse {
    // Get the request body
    let body: serde_json::Value = match http::body(&body) {
        Ok(body) => body,
        Err(_) => {
            return http::response(
                http::Status::BAD_REQUEST,
                serde_json::json!({
                    "response": "Invalid request body"
                }),
            )
        }
    };

    // Get the request body variables
    let user_id: &str = match body["user_id"].as_str() {
        Some(id) => id,
        None => {
            return http::response(
                http::Status::BAD_REQUEST,
                serde_json::json!({
                    "response": "Invalid request body"
                }),
            )
        }
    };

    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => {
            return http::response(
                http::Status::BAD_REQUEST,
                serde_json::json!({
                    "response": "Invalid request"
                }),
            )
        }
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = http::header(&req, "authorization");
    let access_token: String = http::header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            }),
        );
    }

    // Insert the whitelist data into the database
    return match db.insert_class_whitelist(&bearer, &class_id, user_id).await {
        true => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "Student added to whitelist"
            }),
        ),
        false => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to add student to whitelist"
            }),
        ),
    };
}

// The remove_user_from_whitelist() endpoint is used to delete the 
// provided user from the provided class_id's whitelist. Anyone within 
// this whitelist can access the given class_id.
#[actix_web::delete("/class/{class_id}/whitelist/{user_id}")]
async fn remove_user_from_whitelist(req: HttpRequest, db: web::Data<Database>) -> HttpResponse {
    // Get the class id from the url parameters
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => {
            return http::response(
                http::Status::BAD_REQUEST,
                serde_json::json!({
                    "response": "Invalid request"
                }),
            )
        }
    };

    // Get the user id from the url parameters
    let user_id: &str = match req.match_info().get("user_id") {
        Some(id) => id,
        None => {
            return http::response(
                http::Status::BAD_REQUEST,
                serde_json::json!({
                    "response": "Invalid request"
                }),
            )
        }
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = http::header(&req, "authorization");
    let access_token: String = http::header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            }),
        );
    }

    // Delete the whitelist data into the database
    return match db
        .remove_user_from_whitelist(&bearer, &class_id, user_id)
        .await
    {
        true => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "Student removed from whitelist"
            }),
        ),
        false => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to remove student from whitelist"
            }),
        ),
    };
}
