// Import Libraries
mod lib;
use lib::database::Database;
use lib::endpoints::users::endp_users;
use lib::endpoints::classes::endp_classes;
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
            
            // User data
            .service(endp_users::get_user_data)
            .service(endp_users::update_user_data)
            .service(endp_users::insert_user_data)

            // Class data
            .service(endp_classes::update_class_data)
            .service(endp_classes::get_class_data)
            .service(endp_classes::insert_class_data)

            // Class Units
            .service(endp_classes::add_class_unit)
            .service(endp_classes::delete_class_unit)
            .service(endp_classes::update_class_unit)

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