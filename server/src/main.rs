use std::sync::Mutex;
use rusqlite::{Connection, Result};
#[macro_use] extern crate rocket;

// Database Struct for globalizing it's
// connection variable
struct Database {
    conn: rusqlite::Connection,
}

// Store The User data as a struct
struct User {
    hash: String,   // The user hash (aka: the user id)
    name: String,   // The user name
    rsl: i8,        // Whether the user has "Require Student Login" Enabled
    analytics: i8,  // Whether the user has "Analytics" Enabled
}

// Database Implemenetation that contains all the
// functions for manipulating the sqlite db data
impl Database {
    // The function to create the primary database table
    // and add a test value into said table
    fn establish_database(&self) -> Result<()> {
        // Create New Database: (user_hash, user_name, require_student_login[bool], analytics[bool])
        let _ = &self.conn.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, user_hash TEXT NOT NULL, user_name TEXT NOT NULL, rsl INTEGER, analytics INTEGER)", 
            rusqlite::params![]
        )?;
        // Insert test value into the database table
        let _ = &self.conn.execute(
            "INSERT INTO users (user_hash, user_name, rsl, analytics) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["f3371213005fdb16aa1811e915957ec90e668a05bc035dd7b2167aa85e96930d", "heytristaann", 0, 0]
        )?;
        return Ok(())
    }

    // The query_users() function is used to
    // query all the users within the database and print
    // their user_hashes to the screen. This function is primarily
    // used for testing purposes.
    fn query_users(&self) -> Result<String> {
        let prep = &mut self.conn.prepare("SELECT user_hash, user_name, rsl, analytics FROM users")?;
        let users = prep.query_map([], |row| {
            Ok(User {
                hash: row.get(0)?,
                name: row.get(1)?,
                rsl: row.get(2)?,
                analytics: row.get(3)?,
            })
        })?;

        // Print the user hash id's
        let _ = users.into_iter().map(|u| {
            let _user = u.unwrap();
            println!("Found user {:?}: {:?}", _user.hash, _user.name);
        });

        // Return success notifier
        return Ok("test".to_string())
    }
}

// Endpoints
#[get("/<user_hash>/<auth_token>")]
fn verify(db: &rocket::State<Mutex<Database>>, user_hash: &str, auth_token: &str) -> String {
    let db = db.lock().expect("shared state lock");
    let r = db.query_users();

    // Return the user_hash and auth_token
    return format!("User Hash: {}\nAuth Token: {}\nTest: {}", user_hash, auth_token, r.unwrap())
}

// Launch Endpoints
#[launch]
fn rocket() -> _ {
    let db: Database = Database { conn: Connection::open_in_memory().unwrap() };
    let _ = db.establish_database();

    // Build the API Endpoints
    rocket::build()
        .manage(Mutex::new(db))
        .mount("/", routes![verify])
}