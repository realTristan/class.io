use crate::lib::{
    self, global, handlers::Database
};
use actix_web::{web, HttpRequest, Responder};

// The GET /user/<bearer> endpoint is used
// to get an users dashboard settings through their
// bearer. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
//  a valid auth token is required.
#[actix_web::get("/user/{user_id}")]
pub async fn get_user_data(req: HttpRequest, db: web::Data<Database>) -> impl Responder 
{
    // Get the class id
    let user_id: &str = match req.match_info().get("user_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid user id"
        }).to_string()
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // Once the request has been verified, query the
    // database for the provided user_id. Once found,
    // return all the data from said user.
    let user = match db.query_user_by_id(&user_id).await {
        Some(v) => v,
        None => return serde_json::json!({
            "status": 400,
            "response": "Failed to fetch user data"
        }).to_string()
    };

    // Return a formatted string as a json map
    // so the frontend can successfully read the
    // response data.
    return serde_json::json!({
        "status": 200,
        "response": {
            "user_name": user.user_name,
            "user_id": user_id,
            "classes": "array of the users class_ids (select from classes where user_id = user_id)"
        }
    }).to_string()
}

// The POST /user/{bearer} endpoint is used
// to get an users dashboard settings through their
// bearer. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
// a valid auth token is required.
#[actix_web::post("/user")]
pub async fn update_user_data(
    req: HttpRequest, db: web::Data<Database>, body: web::Bytes
) -> impl Responder {
    // Get the request body
    let body: serde_json::Value = match global::get_body(&body) {
        Ok(body) => body,
        Err(_) => return serde_json::json!({
            "status": 400,
            "response": "Invalid request body"
        }).to_string()
    };

    // Get the user name from the request body
    let user_name: String = match body.get("user_name") {
        Some(name) => name.to_string(),
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid user_name"
        }).to_string()
    };
    

    // Get the bearer and access token from the request headers.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // If the incoming request doesn't contain
    // a new user_name, return an empty json map
    if user_name.len() < 1 {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request body"
        }).to_string()
    }

    // Update the username and return whether the update
    // was successful or not
    return match db.update_user_name(&bearer, &user_name).await {
        true => serde_json::json!({
            "status": 200,
            "response": "Successfully updated user data"
        }).to_string(),
        false => serde_json::json!({
            "status": 400,
            "response": "Failed to update user data"
        }).to_string()
    }
}

// The insert_user_data() function is used to insert
// a new row into the users column within the database
// containing the users unique hash, provided name,
// provided email and the current date as the registration time.
// This endpoint is called whenever an user logs into the website
// using firebase google auth.
#[actix_web::put("/user")]
async fn insert_user_data(
    req: HttpRequest, db: web::Data<Database>, body: web::Bytes
) -> impl Responder {

    // Get the request body
    let body: serde_json::Value = match global::get_body(&body) {
        Ok(body) => body,
        Err(_) => return serde_json::json!({
            "status": 400,
            "response": "Invalid request body"
        }).to_string()
    };

    // Get the user name from the request body
    let user_name: String = match body.get("user_name") {
        Some(name) => name.to_string(),
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid user_name"
        }).to_string()
    };

    // Get the user email from the request body
    let email = match body.get("email") {
        Some(email) => email.to_string(),
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid email"
        }).to_string()
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // Insert the user into the database
    // Along with this insertion is the bearer, user_name
    // user's email and the time of registration
    return match db.insert_user(&bearer, &user_name, &email).await {
        true => serde_json::json!({
            "status": 200,
            "response": "Successfully inserted user data"
        }).to_string(),
        false => serde_json::json!({
            "status": 400,
            "response": "Failed to insert user data"
        }).to_string()
    }
}
