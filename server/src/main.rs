mod lib;
use lib::database::handler::Database;
use actix_web::{web, App, HttpServer, Responder, HttpRequest};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establish a new database
    let db: Database = Database::init().await;
    // Insert a test user for debugging purposes
    let _ = db.insert_test_user().await;

    // Establish a connection to http://127.0.0.1:8080/
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            // Get the data of an user in the database  (GET)
            .service(get_user_data)
            // Update the data of an user already in the database (POST)
            .service(update_user_data)
            // Insert an user into the database (PUT)
            .service(insert_user_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// The get_header() function is used to bypass
// any invalid header errors. Without this function,
// if an abuser tried to send a request with an
// invalid error, an internal server error would occur.
fn get_header<'a>(req: &'a HttpRequest, header: &str) -> &'a str {
    // Get the header option to check whether the
    // head is valid/present
    let opt_head = req.headers().get(header);
    // If the header is invalid/not-present, return
    // an empty string
    if opt_head.is_none() { return "" }
    // Unwrap the option header and check if
    // it has a valid length. 
    let head_val = opt_head.unwrap();
    // If it doesn't, return an empty string
    if head_val.is_empty() { return "" }
    // Finally return the header as an 
    // unwrapped string
    return head_val.to_str().unwrap()
}

// The /user/info/<user_hash>/<access_token> endpoint is used
// to get an users dashboard settings through their
// user_hash. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
//  a valid auth token is required.
#[actix_web::get("/user/{user_hash}")]
async fn get_user_data(
    req: HttpRequest, db: web::Data<Database>, user_hash: web::Path<String>
) -> impl Responder {
    // Get the access token from the request headers. 
    // This tokens is used to make sure that the incoming 
    // request isn't from an abuser.
    let access_token: &str = get_header(&req, "Access Token");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&user_hash, access_token) { return "{}".to_string()}
    // Once the request has been verified, query the
    // database for the provided user_hash. Once found,
    // return all the data from said user.
    let user: lib::database::users::User = db.query_user_by_hash(&user_hash).await;

    // Return a formatted string as a json map
    // so the frontend can successfully read the
    // response data.
    return format!(
        "{{\"access_token\": \"{}\", \"user_hash\": \"{}\", \"user_name\": \"{}\", \"classes\": {}}}", 
            access_token, user_hash, user.name, "array of the users class_hashes (select from classes where user_hash = user_hash)"
    )
}

// The UserDataBody struct is used to read
// the incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
struct UserDataBody { user_name: String, email: String }
// The GET /user/{user_hash} endpoint is used
// to get an users dashboard settings through their
// user_hash. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
// a valid auth token is required.
#[actix_web::post("/user/{user_hash}")]
async fn update_user_data(
    req: HttpRequest, db: web::Data<Database>, user_hash: web::Path<String>, body: web::Json<UserDataBody>
) -> impl Responder {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let access_token: &str = get_header(&req, "Access Token");
    let bearer_token: &str = get_header(&req, "Authorization");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&user_hash, access_token) { 
        return "{}".to_string()
    }
    // If the user does not provide a valid bearer token,
    // return an empty json map
    let firebase_token: &str = "";
    if !lib::auth::verify_bearer(&user_hash, access_token, bearer_token, firebase_token) { 
        return "{}".to_string()
    }

    // If the incoming request contains a new user_name
    if body.user_name.len() > 0 {
        // Then update the users 'user_name' in the database
        db.update_user_name(&user_hash, &body.user_name).await;
    }
    // Return successful update
    return format!("{{\"success\": {}}}", true)
}

// The insert_user_data() function is used to insert
// a new row into the users column within the database
// containing the users unique hash, provided name,
// provided email and the current date as the registration time.
// This endpoint is called whenever an user logs into the website
// using firebase google auth.
#[actix_web::put("/user/{user_hash}")]
async fn insert_user_data(
    req: HttpRequest, db: web::Data<Database>, user_hash: web::Path<String>, body: web::Json<UserDataBody>
) -> String {
    // Get the access and authentication tokens from
    // the request headers. These tokens are used to make
    // sure that the incoming request isn't from an abuser.
    let access_token: &str = get_header(&req, "Access Token");
    let bearer_token: &str = get_header(&req, "Authorization");

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&user_hash, access_token) { 
        return "{}".to_string()
    }
    // If the user does not provide a valid bearer token,
    // return an empty json map
    let firebase_token: &str = "";
    if !lib::auth::verify_bearer(&user_hash, access_token, bearer_token, firebase_token) { 
        return "{}".to_string()
    }

    // Get the current system time. This is used
    // for inserting the users registration date
    // into the database.
    let time: std::time::Duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap();
    
    // Insert the user into the database
    // Along with this insertion is the user_hash, user_name
    // user's email and the time of registration
    let _ = db.insert_user(
        &user_hash, &body.user_name, 
        &body.email, time.as_secs() as i64
    );
    // Return successful update
    return format!("{{\"success\": {}}}", true)
}

/*

Example 1:
    GET /class/<class_hash>
    headers={"Access Token": ""}

    fn get_class_data(class_hash: &str) {
        return whitelist[String Array], announcements, rsl[bool], analytics[bool], class_name, enable_whitelist[bool]
        "units": [
            "unit_name": {
                "is_locked": bool
                "lessons": [
                    { 
                        "title": "",
                        "description", "",
                        "video_url": "",
                        "etc...": ""
                    }
                ],
            }
        ]
    }


Example 2:
    POST /class/<class_hash>
    headers={"Access Token": "", "Authorization": "Bearer ..."}

    fn update_class_data(analytics, rsl) {
        
    }

Example 3:
    PUT /class/<class_hash>
    headers={"Access Token": "", "Authorization": "Bearer ..."}

    fn create_class() {
        sets { analytics, rsl, units, whitelist, announcements }
        to default values
    }

Example 4:
    PUT /class/units/<class_hash>
    headers={"Access Token": "", "Authorization": "Bearer ..."}

    and

    DELETE /class/<class_hash>
    headers={"Access Token": "", "Authorization": "Bearer ..."}

    fn add_class_unit(unit_name) {
        
    }

    fn remove_class_unit(unit_hash) {
        
    }


Example 5:
    /class/whitelist/add/<class_hash>/auth_crypt(user_hash:access_token:bearer)
    and
    /class/whitelist/remove/<class_hash>/auth_crypt(user_hash:access_token:bearer)

    fn add_class_whitelist(user_to_add, class_hash) {

    }

    fn remove_class_whitelist(user_to_add, class_hash) {

    }


Example 6:
    /class/units/edit/<class_hash>/auth_crypt(user_hash:access_token:bearer)

    fn add_class_whitelist(unit_hash, unit_title, unit_description, and so on..) {

    }


Example 7:
    /submissions/get/<class_hash>/auth_crypt(user_hash:access_token:bearer)

    
    fn get_unit_submissions(
        Optional<unit_hash: &str>,
        Optional<user_hash: &str>
    ) {
        return [
            { submission_date: submission }
        ]
    }

*/