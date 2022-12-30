use crate::lib;

// Database Implementation
impl lib::handlers::Database {
    // The remove_user_from_whitelist() function deletes
    // an user from the provided class's whitelist. This
    // user can no longer access the provided class.
    pub async fn remove_user_from_whitelist(&self, bearer: &str, class_id: &str, user_id: &str) -> bool 
    {
        // Query the database
        let query = sqlx::query!(
            "DELETE FROM whitelists WHERE owner_bearer=? AND class_id=? AND whitelisted_user_id=?",
            bearer, class_id, user_id
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
    pub async fn insert_class_whitelist(
        &self, bearer: &str, class_id: &str, user_id: &str
    ) -> bool {

        // Get the user name from the user id
        let user_name: String = match self.get_user_name_by_id(user_id).await {
            Some(r) => r,
            None => return false
        };

        // Insert the user into the database
        let query =
            sqlx::query!(
            "INSERT INTO whitelists (owner_bearer, class_id, whitelisted_user_name, whitelisted_user_id) VALUES (?, ?, ?, ?)", 
            bearer, class_id, user_name, user_id
        ).execute(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false,
        };
    }
}
