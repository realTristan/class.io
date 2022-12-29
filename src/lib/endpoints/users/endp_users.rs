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
pub async fn get_user_data(req: HttpRequest, db: web::Data<Database>) -> impl Responder 
{
    // Get the class id
    let user_id: &str = match req.match_info().get("user_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": "400",
            "response": "Invalid user id"
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

    // Once the request has been verified, query the
    // database for the provided user_id. Once found,
    // return all the data from said user.
    let user = match db.query_user_by_id(&user_id).await {
        Some(v) => v,
        None => return serde_json::json!({
            "status": "400",
            "response": "Failed to fetch user data"
        }).to_string()
    };

    // Return a formatted string as a json map
    // so the frontend can successfully read the
    // response data.
    return serde_json::json!({
        "status": "200",
        "response": {
            "user_name": user.user_name,
            "user_id": user_id,
            "classes": "array of the users class_ides (select from classes where user_id = user_id)"
        }
    }).to_string()
}

// The POST /user/{bearer} endpoint is used
// to get an users dashboard settings through their
// bearer. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
// a valid auth token is required.
#[actix_web::post("/user/")]
pub async fn update_user_data(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<UserDataBody>
) -> impl Responder {
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

    // If the incoming request doesn't contain
    // a new user_name, return an empty json map
    if body.user_name.len() < 1 {
        return serde_json::json!({
            "status": "400",
            "response": "Invalid request body"
        }).to_string()
    }

    // Update the username and return whether the update
    // was successful or not
    return match db.update_user_name(&bearer, &body.user_name).await {
        true => serde_json::json!({
            "status": "200",
            "response": "Successfully updated user data"
        }).to_string(),
        false => serde_json::json!({
            "status": "400",
            "response": "Failed to update user data"
        }).to_string()
    }
}

// The insert_user_data() function is used to insert
// a new row into the users column within the database
// containing the users unique hash, provided name,
// provided email and the current date as the registration time.
// This endpoint is called whenever an user logs into the website
// using firebase google auth.
#[actix_web::put("/user/")]
async fn insert_user_data(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<UserDataBody>
) -> impl Responder {
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

    // Insert the user into the database
    // Along with this insertion is the bearer, user_name
    // user's email and the time of registration
    return match db.insert_user(&bearer, &body.user_name, &body.email).await {
        true => serde_json::json!({
            "status": "200",
            "response": "Successfully inserted user data"
        }).to_string(),
        false => serde_json::json!({
            "status": "400",
            "response": "Failed to insert user data"
        }).to_string()
    }
}
