// Import Libraries
mod lib;
use lib::endpoints::classes::endp_submission;
use lib::endpoints::classes::endp_whitelist;
use lib::handlers::Database;
use lib::endpoints::users::endp_users;
use lib::endpoints::classes::endp_class;
use lib::endpoints::classes::endp_unit;
use actix_web::{App, HttpServer, web::Data};

// Main Actix-Web function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish a new database
    let db: Database = Database::init().await;
    // Insert a test user for debugging purposes
    let _ = db.insert_test_user().await;
    db.insert_test_class().await;

    // Establish a connection to http://127.0.0.1:8080/
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            
            // User data
            .service(endp_users::get_user_data)
            .service(endp_users::update_user_data)
            .service(endp_users::insert_user_data)

            // Class data
            .service(endp_class::update_class_data)
            .service(endp_class::get_class_data)
            .service(endp_class::insert_class_data)

            // Class Units
            .service(endp_unit::add_class_unit)
            .service(endp_unit::delete_class_unit)
            .service(endp_unit::update_class_unit)

            // Class Whitelist
            .service(endp_whitelist::delete_from_class_whitelist)
            .service(endp_whitelist::add_to_class_whitelist)

            // Class Submissions
            .service(endp_submission::delete_class_submission)
            .service(endp_submission::insert_class_submission)
            .service(endp_submission::get_user_submissions)
            .service(endp_submission::get_class_submissions)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
