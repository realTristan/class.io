#![allow(dead_code)]

// Library Usages
use super::db_handler;

// Store User data as a struct
pub struct User {
    pub id:         i64,        // Row Increment ID
    pub hash:       String,     // The user hash (aka: the user id)
    pub name:       String,     // The user name
}

// Database Implemenetation that contains all the
// functions for manipulating the user db data
impl db_handler::Database {
    // The insert_test_user() function is used to
    // insert a fake user for testing the backend
    // database functions
    pub async fn insert_test_user(&self) -> sqlx::sqlite::SqliteQueryResult {
        return sqlx::query!( 
            "INSERT INTO users (user_hash, user_name) VALUES (?, ?)",
            "822f3d5b9c91b570a4f1848c5d147b4709d2fb96", "realtristan"
        ).execute(&self.conn).await.unwrap();
    }

    // The query_user_by_hash() function is used to query
    // the database for an user with the provided hash
    // Once found, the function will return the users
    // name, hash, and id
    pub async fn query_user_by_hash(&self, user_hash: &str) -> User {
        // Query the database
        let r = sqlx::query!(
            "SELECT id, user_name FROM users WHERE user_hash=?",
            user_hash
        ).fetch_one(&self.conn).await.unwrap();

        // Return the 'User' object containing all of
        // the requested user's data
        return User {
            id: r.id,
            hash: user_hash.to_string(),
            name: r.user_name
        }
    }
}