#[macro_use] extern crate rocket;

// Import the database functions
#[path = "./database/db.rs"] mod sql;
type Database = rocket::State<sql::Database>;

// Get user by user_hash endpoint
// /822f3d5b9c91b570a4f1848c5d147b4709d2fb96/no_auth
#[get("/<user_hash>/<auth_token>")]
pub async fn verify(db: &Database, user_hash: &str, auth_token: &str) -> String {
    let user: sql::User = db.query_user_by_hash(user_hash).await;

    // Return a json map containing all the user data
    return format!(
        "{{\"auth_token\": \"{}\", \"user_hash\": \"{}\", \"user_name\": \"{}\", \"user_rsl\": {}, \"user_analytics\": {}}}", 
            auth_token, user_hash, user.name, user.rsl==1, user.analytics==1
    )
}
// https://github.com/diesel-rs/diesel

// Launch Endpoints
#[launch]
async fn rocket() -> _ {
    // Establish a new database
    let db: sql::Database = sql::Database::init().await;
    let _ = db.insert_test_user().await;

    // Build the API Endpoints
    rocket::build()
        .manage(db)
        .mount("/", routes![verify])
}