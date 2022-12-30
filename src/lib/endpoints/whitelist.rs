use crate::lib::{
    self, global, handlers::Database
};
use actix_web::{web, HttpRequest, Responder};

// The add_user_to_whitelist() endpoint is used
// to add an user to the provided class_id's
// whitelist. Anyone within this whitelist can
// access the provided class_id. The whitelist
// feature only works if the user has enabled the
// class whitelist setting.
#[actix_web::put("/class/{class_id}/whitelist/")]
async fn add_user_to_whitelist(
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

    // Get the request body variables
    let user_id: &str = match body["user_id"].as_str() {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid user_id"
        }).to_string()
    };

    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
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
    
    // Insert the whitelist data into the database
    return match db.insert_class_whitelist(&bearer, &class_id, user_id).await {
        true => serde_json::json!({
            "status": 200,
            "response": "Success"
        }).to_string(),
        false => serde_json::json!({
            "status": 400,
            "response": "Failed to insert whitelist data"
        }).to_string()
    };
}


// The remove_user_from_whitelist() endpoint is used
// to delete the provided user from the provided
// class_id's whitelist. Anyone within this whitelist
// can access the given class_id.
#[actix_web::delete("/class/{class_id}/whitelist/{user_id}/")]
async fn remove_user_from_whitelist(req: HttpRequest, db: web::Data<Database>) -> impl Responder 
{
    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };

    // Get the user id
    let user_id: &str = match req.match_info().get("user_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
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

    // Delete the whitelist data into the database
    return match db.remove_user_from_whitelist(&bearer, &class_id, user_id).await {
        true => serde_json::json!({
            "status": 200,
            "response": "Success"
        }).to_string(),
        false => serde_json::json!({
            "status": 400,
            "response": "Failed to delete whitelist data"
        }).to_string()
    };
}
