use crate::lib::{
    self, global, handlers::Database, http
};
use actix_web::{web, HttpRequest, HttpResponse};

// The insert_class_unit() endpoint is used to create
// a new unit for the provided class. Using the
// provided class_id the function will generate
// a unique unit identifier using the following format:
// SHA256(class_id:current_time)
#[actix_web::put("/class/{class_id}/units")]
async fn insert_class_unit(
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

    // Get the unit name from the request body
    let unit_name: String = match body.get("unit_name") {
        Some(name) => name.to_string(),
        None => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request body"
            })
        )
    };
    
    // Get the class id from the request headers
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

    // Generate a new unit id
    let unit_id: String = global::generate_new_id(&class_id);

    // Insert the unit data into the database
    return match db.insert_class_unit(&bearer, &unit_id, &class_id, &unit_name).await {
        true => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "Unit created",
                "unit_id": unit_id
            })
        ),
        false => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to create unit"
            })
        )
    }
}


// The update_class_unit() endpoint is used to
// modify any data within the unit's database row.
#[actix_web::put("/class/{class_id}/units/{unit_id}")]
async fn update_class_unit(
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

    // Get the class id from the request headers
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            })
        )
    };

    // Get the unit id from the request headers
    let unit_id: &str = match req.match_info().get("unit_id") {
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

    // Update the unit data in the database
    return match db.update_class_unit(&bearer, class_id, unit_id, body).await {
        true => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "Updated unit"
            })
        ),
        false => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to update unit"
            })
        )
    }
}


// The delete_class_unit() function is used to
// delete the provided unit from the database.
#[actix_web::delete("/class/{class_id}/units/{unit_id}")]
async fn delete_class_unit(
    req: HttpRequest, db: web::Data<Database>
) -> HttpResponse {

    // Get the class id from the request headers
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Invalid request"
            })
        )
    };

    // Get the unit id from the request headers
    let unit_id: &str = match req.match_info().get("unit_id") {
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

    // Insert the unit data into the database
    return match db.delete_class_unit(&bearer, class_id, unit_id).await {
        true => http::response(
            http::Status::OK,
            serde_json::json!({
                "response": "Unit deleted"
            })
        ),
        false => http::response(
            http::Status::BAD_REQUEST,
            serde_json::json!({
                "response": "Failed to delete unit"
            })
        )
    }
}
