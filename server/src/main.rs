#[macro_use] extern crate rocket;

// Import the External files
mod lib;
use lib::database::db_handler::Database;
use rocket::serde::{Serialize, Deserialize, json::Json};

// Database State Simplifier
type DBState = rocket::State<Database>;

// Launch Endpoints
#[launch]
async fn rocket() -> _ {
    // Establish a new database
    let db: Database = Database::init().await;
    // Insert a test user for debugging purposes
    let _ = db.insert_test_user().await;

    // Build the API Endpoints
    rocket::build()
        .manage(db)
        // GET Request (Get user data)
        .mount("/user", routes![update_user_data])
        // POST Request (Update user data)
        .mount("/user", routes![get_user_data])
        // PUT Request (Insert new user data)
        .mount("/user", routies![insert_user_data])
}

// The /user/info/<user_hash>/<auth_token> endpoint is used
// to get an users dashboard settings through their
// user_hash. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
//  a valid auth token is required.
#[get("/<user_hash>/<auth_token>")]
pub async fn get_user_data(db: &DBState, user_hash: &str, auth_token: &str) -> String {
    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(user_hash, auth_token) { return "{}".to_string()}
    // Once the request has been verified, query the
    // database for the provided user_hash. Once found,
    // return all the data from said user.
    let user: lib::database::db_users::User = db.query_user_by_hash(user_hash).await;

    // Return a formatted string as a json map
    // so the frontend can successfully read the
    // response data.
    return format!(
        "{{\"auth_token\": \"{}\", \"user_hash\": \"{}\", \"user_name\": \"{}\", \"classes\": {}}}", 
            auth_token, user_hash, user.name, "array of the users class_hashes (select from classes where user_hash = user_hash)"
    )
}

// The UpdateUserDataBody struct is used to read
// the incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserDataBody { user_name: String }
// The /user/info/<user_hash>/<auth_token> endpoint is used
// to get an users dashboard settings through their
// user_hash. This function is necessary for the frontend
// dashboard page. To ensure the security of the endpoint,
// a valid auth token is required.
#[post("/<auth_crypt>", format = "json", data = "<data>")]  // db: &DBState, 
pub async fn update_user_data(db: &DBState, auth_crypt: &str, data: Json<UpdateUserDataBody>) -> String {
    // Extract the user_hash, auth_token, and bearer token
    // from the url provided base64 encoded auth_crypt.
    let tokens: lib::auth::Tokens = lib::auth::Tokens::from(auth_crypt);

    // If the user does not provide a valid auth
    // token and is trying to abuse the api, return
    // an empty json map
    if !lib::auth::verify(&tokens.user_hash, &tokens.auth_token) { 
        return "{}".to_string()
    }
    // If the user does not provide a valid bearer token,
    // return an empty json map
    let firebase_token: &str = "";
    if !lib::auth::verify_bearer(&tokens.user_hash, &tokens.auth_token, &tokens.bearer, firebase_token) { 
        return "{}".to_string()
    }

    // If the incoming request contains a new user_name
    if data.user_name.len() > 0 {
        // Then update the users 'user_name' in the database
        db.update_user_name(&tokens.user_hash, &data.user_name).await;
    }

    // Return successful update
    return format!("{{\"success\": {}}}", true)
}



// Here there is
// - Whitelists
// - Submissions
// - Classes
// - Units
// - Announcements

/*

Example 1:
    /class/get/<class_hash>/<auth_token>

    fn get_class_data(class_hash: &str) {
        return whitelist[String Array], announcements, rsl, analytics, class_name, 
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
    /class/update/<class_hash>/auth_crypt(user_hash:auth_token:bearer)

    fn update_class_data(analytics, rsl) {
        
    }

Example 3:
    /class/create/auth_crypt(user_hash:auth_token:bearer)

    fn create_class() {
        sets { analytics, rsl, units, whitelist, announcements }
        to default values
    }

Example 4:
    /class/units/add/auth_crypt(user_hash:auth_token:bearer)
    and
    /class/units/remove/auth_crypt(user_hash:auth_token:bearer)

    fn add_class_unit(unit_name) {
        
    }

    fn remove_class_unit(unit_hash) {
        
    }


Example 5:
    /class/whitelist/add/<class_hash>/auth_crypt(user_hash:auth_token:bearer)
    and
    /class/whitelist/remove/<class_hash>/auth_crypt(user_hash:auth_token:bearer)

    fn add_class_whitelist(user_to_add, class_hash) {

    }

    fn remove_class_whitelist(user_to_add, class_hash) {

    }


Example 6:
    /class/units/edit/<class_hash>/auth_crypt(user_hash:auth_token:bearer)

    fn add_class_whitelist(unit_hash, unit_title, unit_description, and so on..) {

    }


Example 7:
    /submissions/get/<class_hash>/auth_crypt(user_hash:auth_token:bearer)

    
    fn get_unit_submissions(
        Optional<unit_hash: &str>,
        Optional<user_hash: &str>
    ) {
        return [
            { submission_date: submission }
        ]
    }

*/