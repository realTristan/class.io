use crate::lib::{
    self, global, handlers::Database, structs::WhitelistDataBody
};
use actix_web::{web, HttpRequest, Responder};

// The add_to_class_whitelist() endpoint is used
// to add an user to the provided class_id's
// whitelist. Anyone within this whitelist can
// access the provided class_id. The whitelist
// feature only works if the user has enabled the
// class whitelist setting.
#[actix_web::put("/class/{class_id}/whitelist")]
async fn add_to_class_whitelist(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<WhitelistDataBody>
) -> impl Responder {

    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };
    
    // Get the access token from the request headers.
    // This tokens is used to make sure that the incoming
    // request isn't from an abuser.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");
    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an invalid request response json
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }
    
    // Insert the whitelist data into the database
    return match db.insert_class_whitelist(&bearer, &class_id, &body.user).await {
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

#[actix_web::delete("/class/{class_id}/whitelist")]
async fn delete_from_class_whitelist(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<WhitelistDataBody>
) -> impl Responder {
    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };

    // Get the access token from the request headers.
    // This tokens is used to make sure that the incoming
    // request isn't from an abuser.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");
    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an invalid request response json
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // Delete the whitelist data into the database
    return match db.delete_from_class_whitelist(&bearer, &class_id, &body.user).await {
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
