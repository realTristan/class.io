use crate::lib;
use actix_web::{web, HttpRequest, Responder};
use lib::global;
use lib::handlers::Database;

// The ClassDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct ClassDataBody {
    // Update the class name
    pub class_name: String,
    // Update whether to use the class whitelist
    pub enable_whitelist: i64,
}

// The get_class_data() endpoint is used to get the class's
// whitelist[String Array], announcements, class_name,
// enable_whitelist[bool], etc.
#[actix_web::get("/class/{class_id}")]
async fn get_class_data(req: HttpRequest, db: web::Data<Database>) -> impl Responder 
{
    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": "400",
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
            "status": "400",
            "response": "Invalid request"
        }).to_string()
    }

    // Return the class data
    return match db.get_class_data(&class_id).await {
        Some(data) => serde_json::json!({
            "status": "200",
            "response": data
        }).to_string(),
        None => serde_json::json!({
            "status": "400",
            "response": "Unable to fetch class data"
        }).to_string()
    }
}

// The update_class_data() endpoint is used to
// modify any one of the Class struct's data.
// This endpoint is utilized within the class dashboard
// and requires a special bearer token to work.
#[actix_web::post("/class/{class_id}")]
async fn update_class_data(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<ClassDataBody>
) -> impl Responder {
    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": "400",
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
            "status": "400",
            "response": "Invalid request"
        }).to_string()
    }

    // Generate a class update query which is the fastest way
    // for updating multiple values inside the database before
    // executing the database update using the below function
    return match db.update_class_data(&bearer, &class_id, &body).await {
        true => serde_json::json!({
            "status": "200",
            "response": "Successfully updated class data"
        }).to_string(),
        false => serde_json::json!({
            "status": "400",
            "response": "Failed to update class data"
        }).to_string()
    }
}

// The insert_class_data() endpoint is used to
// create a new class that contains all the default
// values. A Maximum of 5 classes is allowed.
#[actix_web::put("/class/{class_id}")]
async fn insert_class_data(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<ClassDataBody>
) -> impl Responder {
    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": "400",
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
            "status": "400",
            "response": "Invalid request"
        }).to_string()
    }

    // If the body class_name is invalid,
    // return an empty json map
    if body.class_name.len() < 1 {
        return serde_json::json!({
            "status": "400",
            "response": "Invalid request"
        }).to_string()
    }

    // Insert the class data into the database
    return match db.insert_class_data(&bearer, &class_id, &body.class_name).await {
        true => serde_json::json!({
            "status": "200",
            "response": "Successfully inserted class data"
        }).to_string(),
        false => serde_json::json!({
            "status": "400",
            "response": "Failed to insert class data"
        }).to_string()
    }
}
