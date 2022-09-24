#[macro_use] extern crate rocket;

// Import the database functions
#[path = "./database/db.rs"] mod sql;
type DatabaseState = rocket::State<std::sync::Mutex<sql::Database>>;


// Get user by user_hash endpoint
// /822f3d5b9c91b570a4f1848c5d147b4709d2fb96/no_auth
#[get("/<user_hash>/<auth_token>")]
pub fn verify(db: &DatabaseState, user_hash: &str, auth_token: &str) -> String {
    // Lock the database state mutex
    let db = db.lock().expect("shared state lock");
    // Query the user
    let r = db.query_user_by_hash(user_hash);
    // Return the user_hash and auth_token
    return format!("User Hash: {}\nAuth Token: {}\nUser Data: {:?}", user_hash, auth_token, r.unwrap())
}

// Launch Endpoints
#[launch]
fn rocket() -> _ {
    // Establish a new database
    let db: sql::Database = sql::Database::init().unwrap();
    let _ = db.establish_database();

    // Build the API Endpoints
    rocket::build()
        .manage(std::sync::Mutex::new(db))
        .mount("/", routes![verify])
}