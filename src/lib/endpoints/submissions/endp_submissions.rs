use crate::lib;
use actix_web::{web, HttpRequest, Responder};
use lib::global;
use lib::handlers::Database;

// The SubmissionDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct SubmissionDataBody {
    pub submission_id: String,
    pub data: String,
}

// The get_class_submissions() endpoint is used to
// get all the work submissions for the provided class
// hash. This endpoint is used in the website dashboard
// so that teachers can see all the work their students
// have submitted.
#[actix_web::get("/class/{class_id}/submissions/")]
async fn get_class_submissions(
    req: HttpRequest,
    db: web::Data<Database>,
    class_id: web::Path<String>,
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&bearer, &access_token) {
        return "{}".to_string();
    }
    return db.get_class_submissions(&class_id).await;
}

// The get_user_submissions() endpoint is used to
// retrieve all the submissions from a specific
// user hash from within the database. This endpoint
// is called for the student to see all of their
// previous work submissions.
#[actix_web::get("/class/{class_id}/submissions/")]
async fn get_user_submissions(
    req: HttpRequest,
    db: web::Data<Database>,
    class_id: web::Path<String>,
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&bearer, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }
    return db.get_user_submissions(&class_id, &bearer).await;
}

// The insert_class_submission() endpoint is used
// to insert a new submission into the database.
// This endpoint requires a bearer token, therefore
// the student submitting their work must be signed in.
#[actix_web::put("/class/{class_id}/submissions/{submission_id}/")]
async fn insert_class_submission(
    req: HttpRequest,
    db: web::Data<Database>,
    class_id: web::Path<String>,
    submission_id: web::Path<String>,
    body: web::Json<SubmissionDataBody>,
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let submitter_bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&submitter_bearer, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Insert the submission data into the database
    let r: u64 = db
        .insert_class_submission(&class_id, &submission_id, &submitter_bearer, &body.data)
        .await;

    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0);
}

// The delete_class_submission() function is used to
// delete a submission from the database. This endpoint
// is called when the signed in student wants to undo
// their work submission.
#[actix_web::delete("/class/{class_id}/submissions/{submission_id}/")]
async fn delete_class_submission(
    req: HttpRequest,
    db: web::Data<Database>,
    submission_id: web::Path<String>,
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let submitter_bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&submitter_bearer, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Delete the submission data from the database
    let r: u64 = db
        .delete_class_submission(&submitter_bearer, &submission_id)
        .await;

    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0);
}
