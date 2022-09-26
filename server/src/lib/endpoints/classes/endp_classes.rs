use actix_web::{web, Responder, HttpRequest};
use lib::database::Database;
use lib::global;
use crate::lib;

// Store class announcements
struct Announcement {
    author_name: String,
    title: String,
    description: String,
    attachements: Vec<String>   // Base64 encode images, etc.

}

// Store the class data
struct Class {
    owner_hash: String,
    class_hash: String,
    class_name: String,
    whitelist: Vec<String>,
    announcements: Vec<Announcement>,
    require_student_login: bool,
    analytics: bool,
    units: Vec<Unit>
}

// Store the unit data
struct Unit {
    unit_hash: String,
    unit_name: String,
    locked: bool,
    lessons: Vec<Lesson>
}

// Store the lesson data
struct Lesson {
    unit_hash: String,
    title: String,
    description: String,
    video_url: String,
    work: Vec<String>,
    work_solutions: Vec<String>
}

// The ClassDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
struct ClassDataBody {
    owner_hash: String,
    class_hash: String,
    class_name: String,
    analytics: bool,
    enable_whitelist: bool,
    req_student_login: bool
}
// The UnitDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
struct UnitDataBody {
    class_hash: String,
    unit_hash: String,
    unit_name: String,
    locked: bool
}

// To get how many submissions a student has done, do 
//      SELECT submitter_hash FROM submissions WHERE class_hash=? AND submitter_hash=?


/*

use bearer token when updating the database
and
also use the auth token so the user can't abuse the api

BEARER TOKEN IS SHA256 ENCODE [ (user_hash):(super_secret_bearer_code):(provided auth token):(registration_date) ]

*/


// 
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
    return format!("")
}

// 
#[actix_web::post("/class/{class_hash}")]
async fn update_class_data(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<ClassDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let access_token: &str = global::get_header(&req, "Access Token");
    let bearer_token: &str = global::get_header(&req, "Authorization");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&class_hash, access_token) { 
        return "{}".to_string()
    }
    // If the user does not provide a valid bearer token,
    // return an empty json map
    let firebase_token: &str = "";
    if !lib::auth::verify_bearer(&class_hash, access_token, bearer_token, firebase_token) { 
        return "{}".to_string()
    }
    return format!("")
}

// 
#[actix_web::put("/class/{class_hash}")]
async fn insert_class_data(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<ClassDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let access_token: &str = global::get_header(&req, "Access Token");
    let bearer_token: &str = global::get_header(&req, "Authorization");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&class_hash, access_token) { 
        return "{}".to_string()
    }
    // If the user does not provide a valid bearer token,
    // return an empty json map
    let firebase_token: &str = "";
    if !lib::auth::verify_bearer(&class_hash, access_token, bearer_token, firebase_token) { 
        return "{}".to_string()
    }
    /*
        return whitelist[String Array], announcements, rsl[bool], analytics[bool], class_name, enable_whitelist[bool]
        "units": [
            "unit_name": {
                "is_locked": bool
                "lessons": [
                    { 
                        "title": "",
                        "description", "",
                        "video_url": "",
                        "etc...": ""
                    }
                ],
            }
        ]
    */
    return format!("")
}

// 
#[actix_web::put("/class/units/{unit_hash}")]
async fn add_class_unit(
    req: HttpRequest, db: web::Data<Database>, unit_hash: web::Path<String>, body: web::Json<UnitDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let access_token: &str = global::get_header(&req, "Access Token");
    let bearer_token: &str = global::get_header(&req, "Authorization");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&unit_hash, access_token) { 
        return "{}".to_string()
    }
    // If the user does not provide a valid bearer token,
    // return an empty json map
    let firebase_token: &str = "";
    if !lib::auth::verify_bearer(&unit_hash, access_token, bearer_token, firebase_token) { 
        return "{}".to_string()
    }
    return format!("")
}

// The delete_class_unit() function is used to
// delete the provided unit from the database.
#[actix_web::delete("/class/units/{unit_hash}")]
async fn delete_class_unit(
    req: HttpRequest, db: web::Data<Database>, unit_hash: web::Path<String>, body: web::Json<UnitDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let access_token: &str = global::get_header(&req, "Access Token");
    let bearer_token: &str = global::get_header(&req, "Authorization");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&unit_hash, access_token) { 
        return "{}".to_string()
    }
    // If the user does not provide a valid bearer token,
    // return an empty json map
    let firebase_token: &str = "";
    if !lib::auth::verify_bearer(&unit_hash, access_token, bearer_token, firebase_token) { 
        return "{}".to_string()
    }
    return format!("")
}

// 
#[actix_web::post("/class/units/{unit_hash}")]
async fn update_class_unit(
    req: HttpRequest, db: web::Data<Database>, unit_hash: web::Path<String>, body: web::Json<UnitDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let access_token: &str = global::get_header(&req, "Access Token");
    let bearer_token: &str = global::get_header(&req, "Authorization");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&unit_hash, access_token) { 
        return "{}".to_string()
    }
    // If the user does not provide a valid bearer token,
    // return an empty json map
    let firebase_token: &str = "";
    if !lib::auth::verify_bearer(&unit_hash, access_token, bearer_token, firebase_token) { 
        return "{}".to_string()
    }
    return format!("")
}
