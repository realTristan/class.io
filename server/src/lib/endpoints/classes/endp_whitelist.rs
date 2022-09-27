use actix_web::{web, Responder, HttpRequest};
use lib::handlers::Database;
use lib::global;
use crate::lib;

// The WhitelistDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct WhitelistDataBody { user: String }

// The add_to_class_whitelist() endpoint is used
// to add an user to the provided class_hash's
// whitelist. Anyone within this whitelist can
// access the provided class_hash. The whitelist
// feature only works if the user has enabled the
// class whitelist setting.
#[actix_web::put("/class/{class_hash}/whitelist")]
async fn add_to_class_whitelist(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<WhitelistDataBody>
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
    return format!("")
}

#[actix_web::delete("/class/{class_hash}/whitelist")]
async fn delete_from_class_whitelist(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<WhitelistDataBody>
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
    return format!("")
}
