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
async fn get_class_data(
    req: HttpRequest,
    db: web::Data<Database>,
    class_id: web::Path<String>,
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&bearer, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }
    // Return the class data
    return db.get_class_data(&class_id).await;
}

// The update_class_data() endpoint is used to
// modify any one of the Class struct's data.
// This endpoint is utilized within the class dashboard
// and requires a special bearer token to work.
#[actix_web::post("/class/{class_id}")]
async fn update_class_data(
    req: HttpRequest,
    db: web::Data<Database>,
    class_id: web::Path<String>,
    body: web::Json<ClassDataBody>,
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&bearer, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Generate a class update query which is the fastest way
    // for updating multiple values inside the database before
    // executing the database update using the below function
    let r: u64 = db.update_class_data(&bearer, &class_id, &body).await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0);
}

// The insert_class_data() endpoint is used to
// create a new class that contains all the default
// values. A Maximum of 5 classes is allowed.
#[actix_web::put("/class/{class_id}")]
async fn insert_class_data(
    req: HttpRequest,
    db: web::Data<Database>,
    class_id: web::Path<String>,
    body: web::Json<ClassDataBody>,
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&bearer, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // If the body class_name is invalid,
    // return an empty json map
    if body.class_name.len() < 1 {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Insert the class data into the database
    let r: u64 = db
        .insert_class_data(&bearer, &class_id, &body.class_name)
        .await;

    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0);
}
