// Import Libraries
mod lib;
use lib::database::Database;
use lib::endpoints::users::endp_users;
use actix_web::{App, HttpServer};

// Main Actix-Web function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish a new database
    let db: Database = Database::init().await;
    // Insert a test user for debugging purposes
    let _ = db.insert_test_user().await;

    // Establish a connection to http://127.0.0.1:8080/
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            // Get the data of an user in the database  (GET)
            .service(endp_users::get_user_data)
            // Update the data of an user already in the database (POST)
            .service(endp_users::update_user_data)
            // Insert an user into the database (PUT)
            .service(endp_users::insert_user_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/*

Example 5:
    /class/whitelist/add/<class_hash>/auth_crypt(user_hash:access_token:bearer)
    and
    /class/whitelist/remove/<class_hash>/auth_crypt(user_hash:access_token:bearer)

    fn add_class_whitelist(user_to_add, class_hash) {

    }

    fn remove_class_whitelist(user_to_add, class_hash) {

    }

Example 7:
    /submissions/get/<class_hash>/auth_crypt(user_hash:access_token:bearer)

    
    fn get_unit_submissions(
        Optional<unit_hash: &str>,
        Optional<user_hash: &str>
    ) {
        return [
            { submission_date: submission }
        ]
    }

*/