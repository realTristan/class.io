use crate::lib::{
    self, global, handlers::Database
};
use actix_web::{web, HttpRequest, Responder};

// The get_class_data() endpoint is used to get the class's
// whitelist[String Array], announcements, class_name,
// enable_whitelist[bool], etc.
#[actix_web::get("/class/{class_id}/")]
async fn get_class_data(req: HttpRequest, db: web::Data<Database>) -> impl Responder 
{
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

    // Return the class data
    return match db.get_class_data(&class_id).await {
        Some(data) => serde_json::json!({
            "status": 200,
            "response": data
        }).to_string(),
        None => serde_json::json!({
            "status": 400,
            "response": "Unable to fetch class data"
        }).to_string()
    }
}

// The update_class_data() endpoint is used to
// modify any one of the Class struct's data.
// This endpoint is utilized within the class dashboard
// and requires a special bearer token to work.
#[actix_web::post("/class/{class_id}/")]
async fn update_class_data(
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

    // Generate a class update query which is the fastest way
    // for updating multiple values inside the database before
    // executing the database update using the below function
    return match db.update_class_data(&bearer, class_id, body).await {
        true => serde_json::json!({
            "status": 200,
            "response": "Successfully updated class data"
        }).to_string(),
        false => serde_json::json!({
            "status": 400,
            "response": "Failed to update class data"
        }).to_string()
    }
}

// The insert_class_data() endpoint is used to
// create a new class that contains all the default
// values. A Maximum of 5 classes is allowed.
#[actix_web::put("/class/")]
async fn insert_class_data(
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
    // Get the class name from the request body
    let class_name: String = match body.get("class_name") {
        Some(name) => name.to_string(),
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid class_name"
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

    // Generate a new class id
    let class_id: String = global::generate_new_id(&bearer);

    // Insert the class data into the database
    return match db.insert_class_data(&bearer, &class_id, &class_name).await {
        true => serde_json::json!({
            "status": 200,
            "response": "Successfully inserted class data"
        }).to_string(),
        false => serde_json::json!({
            "status": 400,
            "response": "Failed to insert class data"
        }).to_string()
    }
}
