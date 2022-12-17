use crate::lib;
use actix_web::{web, HttpRequest, Responder};
use lib::global;
use lib::handlers::Database;

// The SubmissionDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct AnnouncementDataBody {
    // The announcements unique identifier
    pub announcement_id: String,
    // The announcement attachment (image, file, etc.)
    pub attachment: String,
    // The announcement content/description
    pub description: String,
    // The title of the announcement
    pub title: String,
    // The name of the user who posted the announcement
    pub author_name: String,
}

// The insert_class_announcement() endpoint is used
// to insert a new announcement into the database.
// A unique announcement identifier is created
// for if the user wants to later delete the post.
#[actix_web::put("/class/{class_id}/announcements/{announcement_id}")]
async fn insert_class_announcement(
    req: HttpRequest,
    db: web::Data<Database>,
    class_id: web::Path<String>,
    announcement_id: web::Path<String>,
    body: web::Json<AnnouncementDataBody>,
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
    let r: u64 = db
        .insert_class_announcement(&bearer, &class_id, &announcement_id, &body)
        .await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0);
}

// The delete_class_announcement() endpoint is used
// to delete an announcement from the database. This
// function requires a bearer token which means the
// user making the announcement must be signed in.
#[actix_web::delete("/class/{class_id}/announcements")]
async fn delete_class_announcement(
    req: HttpRequest,
    db: web::Data<Database>,
    body: web::Json<AnnouncementDataBody>,
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
    let r: u64 = db
        .delete_class_announcement(&bearer, &body.announcement_id)
        .await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0);
}
