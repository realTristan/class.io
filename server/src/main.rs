use actix_web::{App, HttpServer, web::Data};
mod lib;
use lib::{
    endpoints::announcements::endp_announces, 
    endpoints::submissions::endp_submissions,
    endpoints::whitelist::endp_whitelist,
    endpoints::classes::endp_class,
    endpoints::users::endp_users,
    endpoints::units::endp_unit,
    handlers::Database
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
            .service(endp_users::get_user_data)
            .service(endp_users::update_user_data)
            .service(endp_users::insert_user_data)

            // Class data
            .service(endp_class::update_class_data)
            .service(endp_class::get_class_data)
            .service(endp_class::insert_class_data)

            // Class Units
            .service(endp_unit::insert_class_unit)
            .service(endp_unit::delete_class_unit)
            .service(endp_unit::update_class_unit)

            // Class Whitelist
            .service(endp_whitelist::delete_from_class_whitelist)
            .service(endp_whitelist::add_to_class_whitelist)

            // Class Submissions
            .service(endp_submissions::delete_class_submission)
            .service(endp_submissions::insert_class_submission)
            .service(endp_submissions::get_user_submissions)
            .service(endp_submissions::get_class_submissions)

            // Class Announcements
            .service(endp_announces::insert_class_announcement)
            .service(endp_announces::delete_class_announcement)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
