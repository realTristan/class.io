use crate::lib;
use actix_web::{web, HttpRequest, Responder};
use lib::global;
use lib::handlers::Database;

// The UserDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct UserDataBody {
    user_name: String,
    email: String,
}

// The GET /user/<bearer> endpoint is used
// to get an users dashboard settings through their
// bearer. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
//  a valid auth token is required.
#[actix_web::get("/user/{user_id}")]
pub async fn get_user_data(
    req: HttpRequest,
    db: web::Data<Database>,
    user_id: web::Path<String>,
) -> impl Responder {
    // Get the access token from the request headers.
    // This tokens is used to make sure that the incoming
    // request isn't from an abuser.
    let bearer: &str = global::get_header(&req, "authorization");
    let access_token: &str = global::get_header(&req, "access_token");
    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(bearer, access_token) {
        return "{{\"failed\": {}}}".to_string();
    }

    // Once the request has been verified, query the
    // database for the provided user_id. Once found,
    // return all the data from said user.
    let user = db.query_user_by_id(&user_id).await;
    // Check whether or not the user is invalid
    if user.is_none() {
        return "{{\"failed\": {}}}".to_string();
    }
    // Else, if the user is valid, unwrap the
    // object so it can be read
    let user = user.unwrap();

    // Return a formatted string as a json map
    // so the frontend can successfully read the
    // response data.
    return format!(
        "{{\"authorization\": \"{}\", \"user_id\": \"{}\", \"user_name\": \"{}\", \"classes\": {}}}", 
            access_token, user_id, user.user_name, "array of the users class_ides (select from classes where user_id = user_id)"
    );
}

// The POST /user/{bearer} endpoint is used
// to get an users dashboard settings through their
// bearer. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
// a valid auth token is required.
#[actix_web::post("/user/")]
pub async fn update_user_data(
    req: HttpRequest,
    db: web::Data<Database>,
    body: web::Json<UserDataBody>,
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let bearer: &str = global::get_header(&req, "authorization");
    let access_token: &str = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(bearer, access_token) {
        return "{{\"failed\": {}}}".to_string();
    }
    // If the incoming request doesn't contain
    // a new user_name, return an empty json map
    if body.user_name.len() < 1 {
        return "{{\"failed\": {}}}".to_string();
    }
    // Else, update the users 'user_name' in the database
    let r: u64 = db.update_user_name(bearer, &body.user_name).await;

    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0);
}

// The insert_user_data() function is used to insert
// a new row into the users column within the database
// containing the users unique hash, provided name,
// provided email and the current date as the registration time.
// This endpoint is called whenever an user logs into the website
// using firebase google auth.
#[actix_web::put("/user/")]
async fn insert_user_data(
    req: HttpRequest,
    db: web::Data<Database>,
    body: web::Json<UserDataBody>,
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let bearer: &str = global::get_header(&req, "authorization");
    let access_token: &str = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(bearer, access_token) {
        return "{{\"failed\": {}}}".to_string();
    }
    // Get the current system time. This is used
    // for inserting the users registration date
    // into the database.
    let date: i64 = global::get_time() as i64;

    // Insert the user into the database
    // Along with this insertion is the bearer, user_name
    // user's email and the time of registration
    let r: u64 = db
        .insert_user(bearer, &body.user_name, &body.email, date)
        .await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0);
}
