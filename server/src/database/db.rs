
// Database Struct for globalizing it's
// connection variable
pub struct Database { conn: rusqlite::Connection }

// Store The User data as a struct
#[derive(Debug)]
#[allow(dead_code)]
pub struct User {
    id:         i8,         // Row Increment ID
    hash:       String,     // The user hash (aka: the user id)
    name:       String,     // The user name
    rsl:        i8,         // Whether the user has "Require Student Login" Enabled
    analytics:  i8          // Whether the user has "Analytics" Enabled
}

// Database Implemenetation that contains all the
// functions for manipulating the sqlite db data
#[allow(dead_code)]
impl Database {
    // Initialize a new database connection
    pub fn init() -> rusqlite::Result<Database> {
        return Ok(Database{ conn: rusqlite::Connection::open_in_memory()? });
    }

    // The function to create the primary database table
    // and add a test value into said table
    pub fn establish_database(&self) -> rusqlite::Result<()> {
        // Create New Database: (user_hash, user_name, require_student_login[bool], analytics[bool])
        let _ = &self.conn.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, user_hash TEXT NOT NULL, user_name TEXT NOT NULL, rsl INTEGER, analytics INTEGER)", 
            rusqlite::params![]
        )?;
        // Insert test value into the database table
        let _ = &self.conn.execute(
            "INSERT INTO users (user_hash, user_name, rsl, analytics) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params!["822f3d5b9c91b570a4f1848c5d147b4709d2fb96", "heytristaann", 0, 0]
        )?;
        return Ok(())
    }

    // The query_users() function is used to
    // query all the users within the database and print
    // their user_hashes to the screen. This function is primarily
    // used for testing purposes.
    pub fn query_users(&self) -> rusqlite::Result<Vec<User>> {
        // Prepare the sqlite query command
        let prep = &mut self.conn.prepare("SELECT id, user_hash, user_name, rsl, analytics FROM users")?;
        // Iterate over the queried users
        let r = prep.query_map([], |r| {
            Ok(User {
                id:         r.get(0)?,
                hash:       r.get(1)?,
                name:       r.get(2)?,
                rsl:        r.get(3)?,
                analytics:  r.get(4)?,
            })
        })?;
        // Result array
        let res = r.into_iter()
            .map(|f| { f.unwrap()} )
            .collect();

        // Return success notifier
        return Ok(res)
    }

    // The query_user_by_hash() function is used to query
    // the database for an user with the provided hash
    // Once found, the function will return the users
    // name, rsl(bool), analytics(bool) and id
    pub fn query_user_by_hash(&self, user_hash: &str) -> rusqlite::Result<User> {
        // Prepare the sqlite query command
        let prep = &mut self.conn.prepare("SELECT id, user_name, rsl, analytics FROM users WHERE user_hash=:user_hash")?;
        let r = prep.query_row(&[(":user_hash", user_hash)], |r| {
            Ok(User {
                id:         r.get(0)?,
                hash:       user_hash.to_string(),
                name:       r.get(1)?,
                rsl:        r.get(2)?,
                analytics:  r.get(3)?,
            })
        });
        // Return the user
        return Ok(r.unwrap())
    }
}
