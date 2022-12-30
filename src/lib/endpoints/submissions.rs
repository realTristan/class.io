use actix_web::{web, HttpRequest, Responder};
use crate::lib::{
    self, global, handlers::Database
};

// The get_class_submissions() endpoint is used to
// get all the work submissions for the provided class
// hash. This endpoint is used in the website dashboard
// so that teachers can see all the work their students
// have submitted.
#[actix_web::get("/class/{class_id}/submissions")]
async fn get_class_submissions(req: HttpRequest,  db: web::Data<Database>) -> impl Responder 
{
    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // Return the class submissions
    return match db.get_class_submissions(&class_id).await {
        Some(submissions) => serde_json::json!({
            "status": 200,
            "response": submissions
        }).to_string(),
        None => serde_json::json!({
            "status": 400,
            "response": "Failed to fetch class submissions"
        }).to_string()
    }
}

// The get_user_submissions() endpoint is used to
// retrieve all the submissions from a specific
// user hash from within the database. This endpoint
// is called for the student to see all of their
// previous work submissions.
#[actix_web::get("/class/{class_id}/student/submissions")]
async fn get_user_submissions(req: HttpRequest, db: web::Data<Database>) -> impl Responder 
{
    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // Return the user submissions from the database
    return match db.get_user_submissions(&class_id, &bearer).await {
        Some(submissions) => serde_json::json!({
            "status": 200,
            "response": submissions
        }).to_string(),
        None => serde_json::json!({
            "status": 400,
            "response": "Failed to fetch user submissions"
        }).to_string()
    };
}

// The insert_class_submission() endpoint is used
// to insert a new submission into the database.
// This endpoint requires a bearer token, therefore
// the student submitting their work must be signed in.
#[actix_web::put("/class/{class_id}/submissions")]
async fn insert_class_submission(
    req: HttpRequest, db: web::Data<Database>, body: web::Bytes
) -> impl Responder 
{
    // Get the request body
    let body: serde_json::Value = match global::get_body(&body) {
        Ok(body) => body,
        Err(_) => return serde_json::json!({
            "status": 400,
            "response": "Invalid request body"
        }).to_string()
    };

    // Get the submission data from the request body
    let data: String = match body.get("data") {
        Some(data) => data.to_string(),
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid data"
        }).to_string()
    };

    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // Generate a new submission id
    let submission_id: String = global::generate_new_id(&bearer);

    // Insert the submission data into the database
    return match db.insert_class_submission(&class_id, &submission_id, &bearer, &data).await {
        true => serde_json::json!({
            "status": 200,
            "response": "Submission successfully inserted",
            "submission_id": submission_id
        }).to_string(),
        false => serde_json::json!({
            "status": 400,
            "response": "Failed to insert submission"
        }).to_string()
    }
}

// The delete_class_submission() function is used to
// delete a submission from the database. This endpoint
// is called when the signed in student wants to undo
// their work submission.
#[actix_web::delete("/class/{class_id}/submissions/{submission_id}")]
async fn delete_class_submission(req: HttpRequest, db: web::Data<Database>) -> impl Responder 
{
    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };

    // Get the submission id
    let submission_id: &str = match req.match_info().get("submission_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    };

    // Get the bearer and access token from the request headers.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");

    // Verify the provided authorization tokens
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": 400,
            "response": "Invalid request"
        }).to_string()
    }

    // Delete the submission data from the database
    return match db.delete_class_submission(&bearer, &class_id, &submission_id).await {
        true => serde_json::json!({
            "status": 200,
            "response": "Submission successfully deleted"
        }).to_string(),
        false => serde_json::json!({
            "status": 400,
            "response": "Failed to delete submission"
        }).to_string()
    }
}
