use crate::lib;

// The User data struct is used to store
// all of the users data from the database
// into readable values
pub struct User {
    pub id:                 i64,        // Row Increment ID
    pub hash:               String,     // The user hash (aka: the user id)
    pub name:               String,     // The users name
    pub email:              String,     // The users email
    pub registration_date:   i64        // The Users registration date (used for bearer token)
}

// Database Implemenetation that contains all the
// functions for manipulating the user db data
impl lib::handlers::Database {
    // The insert_test_user() function is used to
    // insert a fake user for testing the backend
    // database functions
    pub async fn insert_test_user(&self) -> u64 {
        return self.insert_user(
            "822f3d5b9c91b570a4f1848c5d147b4709d2fb96", "realtristan", 
            "realtristan@gmail.com", 0
        ).await;
    }

    // The insert_user() function is used to insert a
    // new user into the database. Although if the user
    // already exists within the database, the function
    // returns 0 for 0 rows changed.
    pub async fn insert_user(
        &self, user_hash: &str, user_name: &str, email: &str, registration_date: i64
    ) -> u64 {
        // If the user already, exists, return 0
        if self.user_exists(user_hash).await { return 0; }

        // Insert the user into the database
        let r = sqlx::query!(
            "INSERT INTO users (user_hash, user_name, email, registration_date) VALUES (?, ?, ?, ?)",
            user_hash, user_name, email, registration_date
        ).execute(&self.conn).await;
        // If an error has occurred return 0 rows affected
        if r.is_err() { return 0; }
        // Else unwrap the result and return the
        // amount of rows affected
        return r.unwrap().rows_affected();
    }

    // The user_exists() function is used to check whether
    // the provided user_hash is present within the database.
    // If it is, return true.. else return false.
    pub async fn user_exists(&self, user_hash: &str) -> bool {
        // Query the database
        let r = sqlx::query!(
            "SELECT * FROM users WHERE user_hash=?", user_hash
        ).fetch_one(&self.conn).await;
        // Return whether valid query data has been obtained
        return !r.is_err();
    }

    // The query_user_by_hash() function is used to query
    // the database for an user with the provided hash
    // Once found, the function will return the users
    // name, hash, and id
    pub async fn query_user_by_hash(&self, user_hash: &str) -> Option<User> {
        // Query the database
        let r = sqlx::query!(
            "SELECT * FROM users WHERE user_hash=?",
            user_hash
        ).fetch_one(&self.conn).await;

        // If the user is invalid
        if r.is_err() {
            // Return an empty User object
            return None
        }
        // Return the 'User' object containing all of
        // the requested user's data
        let r = r.unwrap();
        return Some(User {
            id: r.id,
            hash: user_hash.to_string(),
            name: r.user_name,
            email: r.email,
            registration_date: r.registration_date
        })
    }

    // The update_user_name() function is used to 
    // modify the incoming users profile name.
    pub async fn update_user_name(&self, user_hash: &str, new_name: &str) -> u64 {
        return sqlx::query!("UPDATE users SET user_name=? WHERE user_hash=?",
            new_name, user_hash
        ).execute(&self.conn).await.unwrap().rows_affected();
    }
}