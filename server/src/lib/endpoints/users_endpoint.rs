#[path = "../database/db_handler.rs"] mod db_handler;
#[path = "../database/db_users.rs"] mod db_users;
#[path = "../auth.rs"] mod auth;

// Import the database functions
type DBState = rocket::State<db_handler::Database>;

// The /user/info/<user_hash>/<auth_token> endpoint is used
// to get an users dashboard settings through their
// user_hash. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
// a valid auth token is required.
#[get("/<user_hash>/<auth_token>")]
pub async fn get_user_data(db: &DBState, user_hash: &str, auth_token: &str) -> String {
    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !auth::verify(user_hash, auth_token) { return "{}".to_string()}
    // Once the request has been verified, query the
    // database for the provided user_hash. Once found,
    // return all the data from said user.
    let user: db_users::User = db.query_user_by_hash(user_hash).await;

    // Return a formatted string as a json map
    // so the frontend can successfully read the
    // response data.
    return format!(
        "{{\"auth_token\": \"{}\", \"user_hash\": \"{}\", \"user_name\": \"{}\", \"classes\": {}}}", 
            auth_token, user_hash, user.name, "array of the users class_hashes (select from classes where user_hash = user_hash)"
    )
}

// The /user/info/<user_hash>/<auth_token> endpoint is used
// to get an users dashboard settings through their
// user_hash. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
// a valid auth token is required.
//
// WARNING:
//  - When making requests to this endpoint, require a Bearer Authentication Token
//
//      - BEARER TOKEN IS SHA256 ENCODE [ (user_hash):(super_secret_bearer_code):(provided auth token):(registration_date) ]
//
#[get("/<user_hash>/<auth_token>")] // db: &DBState, 
pub async fn update_user_data(user_hash: &str, auth_token: &str) -> String {
    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !auth::verify(user_hash, auth_token) { return "{}".to_string()}

    // Return successful update
    return format!("{{\"success\": {}}}", true)
}
