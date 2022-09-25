#[macro_use] extern crate rocket;

// Import the External files
mod lib;
use lib::endpoints::users_endpoint;
use lib::database::db_handler::Database;

// Launch Endpoints
#[launch]
async fn rocket() -> _ {
    // Establish a new database
    let db: Database = Database::init().await;
    // Insert a test user for debugging purposes
    let _ = db.insert_test_user().await;

    // Build the API Endpoints
    rocket::build()
        .manage(db)
        .mount("/user/get", routes![users_endpoint::get_user_data])
        .mount("/user/update", routes![users_endpoint::update_user_data])
}