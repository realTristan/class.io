#[macro_use] extern crate rocket;

// Import the External files
mod lib;
use lib::endpoints;
use lib::database::handler;

// Launch Endpoints
#[launch]
async fn rocket() -> _ {
    // Establish a new database
    let db: handler::Database = handler::Database::init().await;
    // Insert a test user for debugging purposes
    let _ = db.insert_test_user().await;

    // Build the API Endpoints
    rocket::build()
        .manage(db)
        .mount("/user/get", routes![endpoints::users::get_user_data])
        .mount("/user/update", routes![endpoints::users::update_user_data])
}