use actix_web::{web, Responder, HttpRequest};
use lib::handlers::Database;
use lib::global;
use crate::lib;

// The ClassDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct ClassDataBody {
    // The users unique identifier
    pub user_hash: String,
    // Update the class name
    pub class_name: String,
    // Update whether to use the class whitelist
    pub enable_whitelist: i64,
    // Update whether to require student logins
    pub rsl: i64
}

// The get_class_data() endpoint is used to get the class's
// whitelist[String Array], announcements, rsl[bool], 
// class_name, enable_whitelist[bool]
#[actix_web::get("/class/{class_hash}")]
async fn get_class_data(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let access_token: &str = global::get_header(&req, "Access Token");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&class_hash, access_token) { 
        return "{}".to_string()
    }
    // Return the class data
    return db.get_class_data(&class_hash).await;
}

// The update_class_data() endpoint is used to
// modify any one of the Class struct's data.
// This endpoint is utilized within the class dashboard
// and requires a special bearer token to work.
#[actix_web::post("/class/{class_hash}")]
async fn update_class_data(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<ClassDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let access_token: &str = global::get_header(&req, "Access Token");
    let bearer_token: &str = global::get_header(&req, "Authorization");
    let firebase_token: &str = global::get_header(&req, "Google Auth Token");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&class_hash, access_token) { 
        return "{}".to_string()
    }
    // If the user does not provide a valid bearer token,
    // return an empty json map
    if !lib::auth::verify_bearer(&class_hash, access_token, bearer_token, firebase_token) { 
        return "{}".to_string()
    }
    // Generate a class update query which is the fastest way 
    // for updating multiple values inside the database before 
    // executing the database update using the below function
    let r: u64 = db.update_class_data(&class_hash, body).await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0)
}

// The insert_class_data() endpoint is used to
// create a new class that contains all the default
// values. A Maximum of 5 classes is allowed.
#[actix_web::put("/class/{class_hash}")]
async fn insert_class_data(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<ClassDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let access_token: &str = global::get_header(&req, "Access Token");
    let bearer_token: &str = global::get_header(&req, "Authorization");
    let firebase_token: &str = global::get_header(&req, "Google Auth Token");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&class_hash, access_token) { 
        return "{}".to_string()
    }
    // If the user does not provide a valid bearer token,
    // return an empty json map
    if !lib::auth::verify_bearer(&class_hash, access_token, bearer_token, firebase_token) { 
        return "{}".to_string()
    }

    // If the body class_name or user_hash are invalid,
    // return an empty json map
    if body.class_name.len() < 1 || body.user_hash.len() < 1 { 
        return "{}".to_string() 
    }
    // Insert the class data into the database
    let r: u64 = db.insert_class_data(body).await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0)
}