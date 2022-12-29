use crate::lib;

// The Whitelist data struct is used for querying
// the whitelisted users for a specific class.
pub struct Whitelist {
    pub whitelisted_user: String,
}

// Database Implementation
impl lib::handlers::Database {
    // The get_class_whitelist() function is used to return
    // an array containing all the users that are allowed to
    // see the content within the provided class_id
    pub async fn get_class_whitelist(&self, class_id: &str) -> Vec<Whitelist> 
    {
        // Fetch all the whitelisted users that have
        // access to the provided class.
        let query = sqlx::query_as!(
            Whitelist, "SELECT whitelisted_user FROM whitelists WHERE class_id=?", class_id
        ).fetch_all(&self.conn).await;

        // Return the result of the query
        return match query {
            Ok(r) => r,
            Err(_) => Vec::new(),
        }
    }

    // The delete_from_class_whitelist() function deletes
    // an user from the provided class's whitelist. This
    // user can no longer access the provided class.
    pub async fn delete_from_class_whitelist(&self, bearer: &str, class_id: &str, user: &str) -> bool 
    {
        // Query the database
        let query = sqlx::query!(
            "DELETE FROM whitelists WHERE owner_bearer=? AND class_id=? AND whitelisted_user=?",
            bearer, class_id, user
        ).execute(&self.conn).await;

        // Return the result of the query
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false,
        };
    }

    // The insert_class_whitelist() function is used to add an
    // user into the provided class's whitelist. Users in this
    // whitelist can access the class info. The whitelist only
    // works if the teacher has enabled the class whitelist setting
    pub async fn insert_class_whitelist(&self, bearer: &str, class_id: &str, user: &str) -> bool
    {
        // Query the database
        let query =
            sqlx::query!(
            "INSERT INTO whitelists (owner_bearer, class_id, whitelisted_user) VALUES (?, ?, ?)", 
            bearer, class_id, user
        ).execute(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false,
        };
    }

    // The get_whitelist_json() function is used to
    // geterate a new json map as a string from the
    // provided whitelist array.
    pub fn get_whitelist_json(&self, whitelist: Vec<Whitelist>) -> String 
    {
        // Define the json result string
        let mut r: String = String::new();
        // Iterate over the provided whitelisted users array
        // and append each of them to a formatted string array
        whitelist.iter().for_each(|f| {
            r.push_str(&format!(r#""{}","#, f.whitelisted_user));
        });

        // Remove the last comma of the string array
        // before returning the new json map result
        return r[..r.len() - 1].to_string();
    }
}
