use actix_web::{web, Responder, HttpRequest};
use lib::handlers::Database;
use lib::global;
use crate::lib;

// The SubmissionDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct AnnouncementDataBody { 
    pub announcement_hash: String,
    pub submission_hash: String,
    pub data: String,
    pub attachment: String,
    pub description: String,
    pub title: String,
    pub author_name: String
}

//
//
//
#[actix_web::put("/class/{class_hash}/announcements")]
async fn insert_class_announcement(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<AnnouncementDataBody>
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
    //
    // create a new announcement_hash
    //
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", 1 > 0)
}

//
//
//
#[actix_web::delete("/class/{class_hash}/announcements")]
async fn delete_class_announcement(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<AnnouncementDataBody>
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
    //
    // use the announcement_hash
    //
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", 1 > 0)
}