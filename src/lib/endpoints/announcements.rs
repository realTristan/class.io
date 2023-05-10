use crate::lib::{self, utils, handlers::Database, http};
use actix_web::{web, HttpRequest, HttpResponse};

// The insert_class_announcement() endpoint is used
// to insert a new announcement into the database.
// A unique announcement identifier is created
// for if the user wants to later delete the post.
#[actix_web::put("/class/{class_id}/announcements")]
async fn insert_class_announcement(
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
                    "response": "Invalid request"
                }),
            )
        }
    };

    // Get the class id from the request parameters
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

    // Generate a new announcement id
    let announcement_id: String = utils::generate_new_id(&class_id);

    // Insert the announcement into the database
    return match db
        .insert_class_announcement(&bearer, &class_id, &announcement_id, &body)
        .await
    {
        true => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "Announcement successfully created",
                "announcement_id": announcement_id
            }),
        ),
        false => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to create announcement"
            }),
        ),
    };
}

// The delete_class_announcement() endpoint is used
// to delete an announcement from the database. This
// function requires a bearer token which means the
// user making the announcement must be signed in.
#[actix_web::delete("/class/{class_id}/announcements{announcement_id}")]
async fn delete_class_announcement(req: HttpRequest, db: web::Data<Database>) -> HttpResponse {
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

    // Get the announcement id
    let announcement_id: &str = match req.match_info().get("announcement_id") {
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

    // Delete the announcement from the database
    return match db
        .delete_class_announcement(&bearer, class_id, announcement_id)
        .await
    {
        true => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "Announcement succesfully deleted"
            }),
        ),
        false => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to delete announcement"
            }),
        ),
    };
}
