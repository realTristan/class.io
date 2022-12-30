mod lib;
use lib::{
    endpoints, handlers::Database
};
use actix_web::{
    web::Data, App, HttpServer
};

// Main Actix-Web function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish a new database
    let db: Database = Database::init().await;
    // Insert a test user for debugging purposes
    let _ = db.insert_test_user().await;
    // db.insert_test_class().await;

    // Establish a connection to http://127.0.0.1:8080/
    HttpServer::new(move || {
        App::new()
            .wrap(actix_cors::Cors::permissive())
            .app_data(Data::new(db.clone()))
            // User data
            .service(endpoints::users::get_user_data)
            .service(endpoints::users::update_user_data)
            .service(endpoints::users::insert_user_data)
            // Class data
            .service(endpoints::classes::update_class_data)
            .service(endpoints::classes::get_class_data)
            .service(endpoints::classes::insert_class_data)
            // Class Units
            .service(endpoints::units::insert_class_unit)
            .service(endpoints::units::delete_class_unit)
            .service(endpoints::units::update_class_unit)
            // Class Whitelist
            .service(endpoints::whitelist::remove_user_from_whitelist)
            .service(endpoints::whitelist::add_user_to_whitelist)
            // Class Submissions
            .service(endpoints::submissions::delete_class_submission)
            .service(endpoints::submissions::insert_class_submission)
            .service(endpoints::submissions::get_user_submissions)
            .service(endpoints::submissions::get_class_submissions)
            // Class Announcements
            .service(endpoints::announcements::insert_class_announcement)
            .service(endpoints::announcements::delete_class_announcement)
            // Trim path trailing slashes
            .wrap(actix_web::middleware::NormalizePath::trim())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
