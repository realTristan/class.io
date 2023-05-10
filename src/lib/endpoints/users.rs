use crate::lib::{self, handlers::Database, http};
use actix_web::{web, HttpRequest, HttpResponse};

// The GET /users/{user_id} endpoint is used
// to get an users data using their unique user_id
#[actix_web::get("/users/{user_id}")]
pub async fn get_user_data(req: HttpRequest, db: web::Data<Database>) -> HttpResponse {
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

    // Once the request has been verified, query the
    // database for the provided user_id. Once found,
    // return all the data from said user.
    return match db.query_user_by_id(&user_id).await {
        Some(user) => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "status": "200",
                "response": {
                    "user_name": user.user_name,
                    "user_id": user_id,
                    "classes": "array of the users class_ids (select from classes where user_id = user_id)"
                }
            }),
        ),
        None => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to fetch user data"
            }),
        ),
    };
}

// The PUT /user endpoint is used to get an users dashboard settings through their
// bearer. This function is necessary for the frontend dashboard page. To ensure the 
// security of the endpoint, a valid auth token is required.
#[actix_web::put("/users/{user_id}")]
pub async fn update_user_data(
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

    // Get the user name from the request body
    let user_name: String = match body.get("user_name") {
        Some(name) => name.to_string(),
        None => {
            return http::response(
                http::Status::BAD_REQUEST,
                serde_json::json!({
                    "response": "Invalid request body"
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

    // Update the username and return whether the update
    // was successful or not
    return match db.update_user_name(&bearer, &user_name).await {
        true => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "Updated user data"
            }),
        ),
        false => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to update user data"
            }),
        ),
    };
}

// The insert_user_data() function is used to insert a new row into the users 
// column within the database containing the users unique hash, provided name, 
// provided email and the current date as the registration time.
// This endpoint is called whenever an user logs into the website using firebase google auth.
#[actix_web::put("/users")]
async fn insert_user_data(
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

    // Get the user name from the request body
    let user_name: String = match body.get("user_name") {
        Some(name) => name.to_string(),
        None => {
            return http::response(
                http::Status::BAD_REQUEST,
                serde_json::json!({
                    "response": "Invalid request body"
                }),
            )
        }
    };

    // Get the user email from the request body
    let email = match body.get("email") {
        Some(email) => email.to_string(),
        None => {
            return http::response(
                http::Status::BAD_REQUEST,
                serde_json::json!({
                    "response": "Invalid request body"
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

    // Insert the user into the database
    // Along with this insertion is the bearer, user_name
    // user's email and the time of registration
    return match db.insert_user(&bearer, &user_name, &email).await {
        true => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "Inserted user data"
            }),
        ),
        false => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to insert user data"
            }),
        ),
    };
}
