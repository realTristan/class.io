use actix_web::{web, Responder, HttpRequest};
use lib::handlers::Database;
use lib::global;
use crate::lib;

// The WhitelistDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct WhitelistDataBody { pub user: String }

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
    let user: &str = global::get_header(&req, "user");
    let access_token: &str = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&user, &access_token) { 
        return "{}".to_string()
    }
    // Insert the whitelist data into the database
    let r: u64 = db.insert_class_whitelist(&user, &class_hash, body).await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0)
}

#[actix_web::delete("/class/{class_hash}/whitelist")]
async fn delete_from_class_whitelist(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<WhitelistDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let user: &str = global::get_header(&req, "user");
    let access_token: &str = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&user, &access_token) { 
        return "{}".to_string()
    }
    // Delete the whitelist data into the database
    let r: u64 = db.delete_from_class_whitelist(&user, &class_hash, body).await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0)
}
