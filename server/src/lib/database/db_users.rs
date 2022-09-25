// External Crates Usages
use super::db_handler::Database;

// Store User data as a struct
pub struct User {
    pub id:                 i64,        // Row Increment ID
    pub hash:               String,     // The user hash (aka: the user id)
    pub name:               String,     // The users name
    pub email:              String,     // The users email
    pub registration_date:   i64        // The Users registration date (used for bearer token)
}

// Database Implemenetation that contains all the
// functions for manipulating the user db data
impl Database {
    // The insert_test_user() function is used to
    // insert a fake user for testing the backend
    // database functions
    pub async fn insert_test_user(&self) -> sqlx::sqlite::SqliteQueryResult {
        return sqlx::query!( 
            "INSERT INTO users (user_hash, user_name, email, registration_date) VALUES (?, ?, ?, ?)",
            "822f3d5b9c91b570a4f1848c5d147b4709d2fb96", "realtristan", "fake@gmail.com", 0
        ).execute(&self.conn).await.unwrap();
    }

    // The user_exists() function is used to check whether
    // the provided user_hash is present within the database.
    // If it is, return true.. else return false.
    pub async fn user_exists(user_hash: &str) -> bool {
        // Query the database
        let r = sqlx::query!(
            "SELECT * FROM users WHERE user_hash=?",
            user_hash
        ).fetch_one(&self.conn).await;
        // Return whether valid query data has been obtained
        return !r.is_none();
    }

    // The query_user_by_hash() function is used to query
    // the database for an user with the provided hash
    // Once found, the function will return the users
    // name, hash, and id
    pub async fn query_user_by_hash(&self, user_hash: &str) -> User {
        // Query the database
        let r = sqlx::query!(
            "SELECT * FROM users WHERE user_hash=?",
            user_hash
        ).fetch_one(&self.conn).await;

        // If the user is invalid
        if r.is_none() {
            return User {
                id: 0,
                hash_hash: user_hash.to_string(),
                name: "None",
                email: "None",
                registration_date: 0
            }
        }

        // Return the 'User' object containing all of
        // the requested user's data
        let r = r.unwrap();
        return User {
            id: r.id,
            hash: user_hash.to_string(),
            name: r.user_name,
            email: r.email,
            registration_date: r.registration_date
        }
    }

    // The update_user_name() function is used to 
    // modify the incoming users profile name.
    pub async fn update_user_name(&self, user_hash: &str, new_name: &str) {
        sqlx::query!("UPDATE users SET user_name=? WHERE user_hash=?",
            new_name, user_hash
        ).execute(&self.conn).await.unwrap();
    }
}