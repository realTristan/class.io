use crate::lib::{
    self, global, handlers::Database, http
};
use actix_web::{web, HttpRequest, HttpResponse};

// The get_class_data() endpoint is used to get the class's
// whitelist[String Array], announcements, class_name,
// enable_whitelist[bool], etc.
#[actix_web::get("/class/{class_id}")]
async fn get_class_data(req: HttpRequest, db: web::Data<Database>) -> HttpResponse 
{
    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            })
        )
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = http::header(&req, "authorization");
    let access_token: String = http::header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            })
        )
    }

    // Return the class data
    return match db.get_class_data(&class_id).await {
        Some(data) => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": data
            })
        ),
        None => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            })
        )
    }
}

// The update_class_data() endpoint is used to
// modify any one of the Class struct's data.
// This endpoint is utilized within the class dashboard
// and requires a special bearer token to work.
#[actix_web::put("/class/{class_id}")]
async fn update_class_data(
    req: HttpRequest, db: web::Data<Database>, body: web::Bytes
) -> HttpResponse {

    // Get the request body
    let body: serde_json::Value = match http::body(&body) {
        Ok(body) => body,
        Err(_) => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request body"
            })
        )
    };

    // Get the class id from the url parameters
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            })
        )
    };
    
    // Get the bearer and access token from the request headers.
    let bearer: String = http::header(&req, "authorization");
    let access_token: String = http::header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            })
        )
    }

    // Generate a class update query which is the fastest way
    // for updating multiple values inside the database before
    // executing the database update using the below function
    return match db.update_class_data(&bearer, class_id, body).await {
        true => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "Updated class data"
            })
        ),
        false => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to update class data"
            })
        )
    }
}

// The insert_class_data() endpoint is used to
// create a new class that contains all the default
// values. A Maximum of 5 classes is allowed.
#[actix_web::put("/class")]
async fn insert_class_data(
    req: HttpRequest, db: web::Data<Database>, body: web::Bytes
) -> HttpResponse {

    // Get the request body
    let body: serde_json::Value = match http::body(&body) {
        Ok(body) => body,
        Err(_) => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request body"
            })
        )
    };
    // Get the class name from the request body
    let class_name: String = match body.get("class_name") {
        Some(name) => name.to_string(),
        None => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request body"
            })
        )
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = http::header(&req, "authorization");
    let access_token: String = http::header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            })
        )
    }

    // Generate a new class id
    let class_id: String = global::generate_new_id(&bearer);

    // Insert the class data into the database
    return match db.insert_class_data(&bearer, &class_id, &class_name).await {
        true => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "Created class"
            })
        ),
        false => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to create class"
            })
        )
    }
}
