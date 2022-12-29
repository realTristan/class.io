use crate::lib::{
    self, global, handlers::Database, structs::AnnouncementDataBody
};
use actix_web::{web, HttpRequest, Responder};

// The insert_class_announcement() endpoint is used
// to insert a new announcement into the database.
// A unique announcement identifier is created
// for if the user wants to later delete the post.
#[actix_web::put("/class/{class_id}/announcements/{announcement_id}")]
async fn insert_class_announcement(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<AnnouncementDataBody>
) -> impl Responder {
    // Get the class id
    let class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": "400",
            "response": "Invalid request"
        }).to_string()
    };
    // Get the announcement id
    let announcement_id: &str = match req.match_info().get("announcement_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": "400",
            "response": "Invalid request"
        }).to_string()
    };

    // Get the access token from the request headers.
    // This tokens is used to make sure that the incoming
    // request isn't from an abuser.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");
    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an invalid request response json
    if !lib::auth::verify(&bearer, &access_token) {
        return "{\"error\": \"invalid request\"}".to_string();
    }

    // Insert the announcement into the database
    return match db.insert_class_announcement(&bearer, &class_id, &announcement_id, &body).await {
        true => serde_json::json!({
            "status": "200",
            "response": "Announcement successfully created"
        }).to_string(),
        false => serde_json::json!({
            "status": "400",
            "response": "Invalid request"
        }).to_string()
    }
}

// The delete_class_announcement() endpoint is used
// to delete an announcement from the database. This
// function requires a bearer token which means the
// user making the announcement must be signed in.
#[actix_web::delete("/class/{class_id}/announcements")]
async fn delete_class_announcement(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<AnnouncementDataBody>
) -> impl Responder {
    // Get the class id
    let _class_id: &str = match req.match_info().get("class_id") {
        Some(id) => id,
        None => return serde_json::json!({
            "status": "400",
            "response": "Invalid request"
        }).to_string()
    };

    // Get the access token from the request headers.
    // This tokens is used to make sure that the incoming
    // request isn't from an abuser.
    let bearer: String = global::get_header(&req, "authorization");
    let access_token: String = global::get_header(&req, "access_token");
    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an invalid request response json
    if !lib::auth::verify(&bearer, &access_token) {
        return serde_json::json!({
            "status": "400",
            "response": "Invalid request"
        }).to_string()
    }

    // Delete the announcement from the database
    return match db.delete_class_announcement(&bearer, &body.announcement_id).await {
        true => serde_json::json!({
            "status": "200",
            "response": "Announcement succesfully deleted"
        }).to_string(),
        false => serde_json::json!({
            "status": "400",
            "response": "Failed to delete announcement"
        }).to_string()
    }
}
