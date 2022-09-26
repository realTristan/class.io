use actix_web::{web, Responder, HttpRequest};
use lib::database::Database;
use lib::global;
use crate::lib;

//
// The unit hash is just the class_hash:time.now()
// The class hash is in the url
//

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
    // Update whether to require student logins
    pub rsl: i64
}
// The UnitDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct UnitDataBody { unit_name: String, locked: bool }
// The WhitelistDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct WhitelistDataBody { class_hash: String, user: String }

// The SubmissionDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct SubmissionDataBody { submitter_hash: String, data: String }

// The get_class_data() endpoint is used to get the class's
// whitelist[String Array], announcements, rsl[bool], 
// class_name, enable_whitelist[bool]
/* Example:
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
    ]*/
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
    let _  = db.update_class_data(&class_hash, body);

    // Return the success json body
    return "{{\"success\": true}}".to_string()
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

// The add_class_unit() endpoint is used to create
// a new unit for the provided class. Using the 
// provided class_hash the function will generate
// a unique unit identifier using the following format:
// SHA256(class_hash:current_time)
#[actix_web::put("/class/{class_hash}/units")]
async fn add_class_unit(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<UnitDataBody>
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

// The delete_class_unit() function is used to
// delete the provided unit from the database.
#[actix_web::delete("/class/{class_hash}/units")]
async fn delete_class_unit(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<UnitDataBody>
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

// The update_class_unit() endpoint is used to
// modify any data within the unit's database row.
#[actix_web::post("/class/{class_hash}/units")]
async fn update_class_unit(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<UnitDataBody>
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


#[actix_web::put("/class/{class_hash}/whitelist")]
async fn add_to_class_whitelist(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<WhitelistDataBody>
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

#[actix_web::delete("/class/{class_hash}/whitelist")]
async fn delete_from_class_whitelist(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<WhitelistDataBody>
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

#[actix_web::get("/class/{class_hash}/submissions")]
async fn get_class_submissions(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<SubmissionDataBody>
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


#[actix_web::put("/class/{class_hash}/submissions")]
async fn insert_class_submission(
    req: HttpRequest, db: web::Data<Database>, class_hash: web::Path<String>, body: web::Json<SubmissionDataBody>
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