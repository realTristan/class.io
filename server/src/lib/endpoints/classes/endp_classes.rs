use actix_web::{web, Responder, HttpRequest};
use lib::database::Database;
use lib::global;
use crate::lib;

// The Announcement data struct is used to
// store the announcement author's unique identifier,
// the authors name, the announcement title and description,
// along with any attachements the author has posted with it.
struct Announcement {
    // The author's unique identifier (required for bearer token)
    author_hash: String,
    // The announcement's author name
    // Use the get_user_data endpoint to get this
    author_name: String,
    // The announcement title
    title: String,
    // The announcements content
    description: String,
    // Any images/videos attached with the announcement
    attachements: Vec<String>   // Base64 encode images, etc.
}
// The Class data struct is used to store
// the classes owner_hash, unique class identifier,
// class name, class whitelist array, class announcements,
// rsl bool, analytics bool and the class units.
struct Class {
    // The Teacher's unique user identifier
    owner_hash: String,
    // The Classes unique identifier
    class_hash: String,
    // The Class Name
    class_name: String,
    // An array of user hashes that are used to
    // determine whether a student is allowed to
    // access this class
    whitelist: Vec<String>,
    // An array of announcements that use the above struct
    announcements: Vec<Announcement>,
    // Whether the students need to be logged in to
    // access this class
    require_student_login: bool,
    // Whether to show activity graphs and analytics
    // for this class. 
    analytics: bool,
    // An array of units that use the below struct
    units: Vec<Unit>
}
// The Unit data struct is used to store
// the class unit's unique identifier, 
// unit name, it's locked status and the
// lessons that come along with the unit.
struct Unit {
    // The unique unit identifier
    unit_hash: String,
    // The Unit's Name
    unit_name: String,
    // Whether students can access this unit yet
    locked: bool,
    // The Unit lessons that uses the below struct
    lessons: Vec<Lesson>
}
// The Lesson data struct is used to store
// the class unit's lesson title, description,
// video_url, work and work_solutions.
struct Lesson {
    // The unique unit identifier
    unit_hash: String,
    // The Lesson Title
    title: String,
    // The Lesson Description
    description: String,
    // The Lesson's Youtube Video URL
    video_url: String,
    // The Lesson Homework that can be 
    // submitted and marked
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

// The get_class_data() endpoint is used to get the class's
// whitelist[String Array], announcements, rsl[bool], 
// analytics[bool], class_name, enable_whitelist[bool]
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
#[actix_web::put("/class/units/{class_hash}")]
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
#[actix_web::delete("/class/units/{class_hash}")]
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
#[actix_web::post("/class/units/{class_hash}")]
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
