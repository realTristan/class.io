use actix_web::{web, Responder, HttpRequest};
use lib::handlers::Database;
use lib::global;
use crate::lib;

//
// The unit hash is just the class_id:time.now()
// The class hash is in the url
//

// The UnitDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct UnitDataBody { 
    pub unit_id: String,
    pub unit_name: String,
    pub locked: i64
}

// The insert_class_unit() endpoint is used to create
// a new unit for the provided class. Using the 
// provided class_id the function will generate
// a unique unit identifier using the following format:
// SHA256(class_id:current_time)
#[actix_web::put("/class/{class_id}/units/")]
async fn insert_class_unit(
    req: HttpRequest, db: web::Data<Database>, class_id: web::Path<String>, body: web::Json<UnitDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let bearer: &str = global::get_header(&req, "authorization");
    let access_token: &str = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // Generate a new unit hash. Generate a unit hash inside the api
    // so that if someone finds a way to abuse the api, they can't just
    // use an existing unit hash.
    let unit_id: String = global::generate_new_id(&class_id);

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&bearer, &access_token) { 
        return "{}".to_string()
    }
    // Insert the unit data into the database
    let r: u64 = db.insert_class_unit(&bearer, &unit_id, &class_id, &body.unit_name).await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0)
}

// The delete_class_unit() function is used to
// delete the provided unit from the database.
#[actix_web::delete("/class/{class_id}/units/{unit_id}")]
async fn delete_class_unit(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<UnitDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let bearer: &str = global::get_header(&req, "authorization");
    let access_token: &str = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&bearer, &access_token) { 
        return "{}".to_string()
    }
    // Insert the unit data into the database
    let r: u64 = db.delete_class_unit(&bearer, &body.unit_id).await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0)
}

// The update_class_unit() endpoint is used to
// modify any data within the unit's database row.
#[actix_web::post("/class/{class_id}/units")]
async fn update_class_unit(
    req: HttpRequest, db: web::Data<Database>, body: web::Json<UnitDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let bearer: &str = global::get_header(&req, "authorization");
    let access_token: &str = global::get_header(&req, "access_token");
    // the access token consists of the users sha256 encoded firebase token,
    // the current time, and a "super secret key".
    // This also acts as a bearer token from the encoded firebase token
    // which verifies that the user using this endpoint is the owner.

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&bearer, &access_token) { 
        return "{}".to_string()
    }
    // Insert the unit data into the database
    let r: u64 = db.update_class_unit(&bearer, &body).await;
    // Return whether more than 0 rows were affected
    return format!("{{\"success\": {}}}", r > 0)
}