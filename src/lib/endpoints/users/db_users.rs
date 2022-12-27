use crate::lib::{self, global};

// The User data struct is used to store
// all of the users data from the database
// into readable values
pub struct User {
    // Row Increment ID
    pub id: i64,
    // The unique user identifier
    pub user_id: String,
    // The user hash (aka: the user id)
    pub bearer: String,
    // The users name
    pub user_name: String,
    // The users email
    pub email: String,
    // The Users registration date (used for bearer token)
    pub registration_date: i64,
}

// Database Implemenetation that contains all the
// functions for manipulating the user db data
impl lib::handlers::Database {
    // The insert_test_user() function is used to
    // insert a fake user for testing the backend
    // database functions
    pub async fn insert_test_user(&self) -> u64 {
        return self
            .insert_user(
                "822f3d5b9c91b570a4f1848c5d147b4709d2fb96",
                "realtristan",
                "realtristan@gmail.com",
                0,
            )
            .await;
    }

    // The insert_user() function is used to insert a
    // new user into the database. Although if the user
    // already exists within the database, the function
    // returns 0 for 0 rows changed.
    pub async fn insert_user(&self, bearer: &str, user_name: &str, email: &str, date: i64) -> u64 {
        // If the user already, exists, return 0
        if self.user_exists(bearer).await {
            return 0;
        }

        // Generate a new user id
        let user_id: String = global::generate_new_id(&format!("{email}:{bearer}:{date}"));

        // Insert the user into the database
        let r = sqlx::query!(
            "INSERT INTO users (bearer, user_id, user_name, email, registration_date) VALUES (?, ?, ?, ?, ?)",
            bearer, user_id, user_name, email, date
        ).execute(&self.conn).await;

        // Return query result
        return match r {
            Ok(r) => r.rows_affected(),
            Err(_) => 0,
        };
    }

    // The user_exists() function is used to check whether
    // the provided bearer is present within the database.
    // If it is, return true.. else return false.
    async fn user_exists(&self, bearer: &str) -> bool {
        // Query the database
        let r = sqlx::query!(
            "SELECT * FROM users WHERE bearer=?", bearer
        ).fetch_one(&self.conn).await;

        // Return whether valid query data has been obtained
        return !r.is_err();
    }

    // The query_user_by_id() function is used to query
    // the database for an user with the provided hash
    // Once found, the function will return the users
    // name, hash, and id
    pub async fn query_user_by_id(&self, user_id: &str) -> Option<User> {
        // Query the database
        let r = sqlx::query_as!(
            User, "SELECT * FROM users WHERE user_id=?", user_id
        ).fetch_one(&self.conn).await;

        // Return query result
        return match r {
            Ok(r) => Some(r),
            Err(_) => None,
        };
    }

    // The update_user_name() function is used to
    // modify the incoming users profile name.
    pub async fn update_user_name(&self, bearer: &str, new_name: &str) -> u64 {
        let r = sqlx::query!(
            "UPDATE users SET user_name=? WHERE bearer=?",
            new_name, bearer
        ).execute(&self.conn).await;

        // Return query result
        return match r {
            Ok(r) => r.rows_affected(),
            Err(_) => 0,
        };
    }
}
