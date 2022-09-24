#[macro_use] extern crate rocket;

// Import the database functions
#[path = "./database/db.rs"] mod sql;
#[path = "./database/auth.rs"] mod auth;
type DBState = rocket::State<sql::Database>;

// The /user/info/<user_hash>/<auth_token> endpoint is used
// to get an users dashboard settings through their
// user_hash. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
// a valid auth token is required.
#[get("/<user_hash>/<auth_token>")]
pub async fn user_info(db: &DBState, user_hash: &str, auth_token: &str) -> String {
    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !auth::verify(user_hash, auth_token) { return "{}".to_string()}
    // Once the request has been verified, query the
    // database for the provided user_hash. Once found,
    // return all the data from said user.
    let user: sql::User = db.query_user_by_hash(user_hash).await;

    // Return a formatted string as a json map
    // so the frontend can successfully read the
    // response data.
    return format!(
        "{{\"auth_token\": \"{}\", \"user_hash\": \"{}\", \"user_name\": \"{}\", \"user_rsl\": {}, \"user_analytics\": {}}}", 
            auth_token, user_hash, user.name, user.rsl==1, user.analytics==1
    )
}

// Launch Endpoints
#[launch]
async fn rocket() -> _ {
    // Establish a new database
    let db: sql::Database = sql::Database::init().await;
    // Insert a test user for debugging purposes
    let _ = db.insert_test_user().await;

    // Build the API Endpoints
    rocket::build()
        .manage(db)
        .mount("/user/info", routes![user_info])
}