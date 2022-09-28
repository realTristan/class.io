use actix_web::web::Json;
use crate::lib;
use super::endp_whitelist::WhitelistDataBody;

// The Whitelist data struct is used for querying
// the whitelisted users for a specific class.
pub struct Whitelist { pub whitelisted_user: String }

// Database Implementation
impl lib::handlers::Database {
    // The get_class_whitelist() function is used to return
    // an array containing all the users that are allowed to
    // see the content within the provided class_hash
    pub async fn get_class_whitelist(&self, class_hash: &str) -> Vec<Whitelist> {
        // Fetch all the whitelisted users that have
        // access to the provided class.
        let r = sqlx::query_as!(
            Whitelist, "SELECT whitelisted_user FROM whitelists WHERE class_hash=?", class_hash
        ).fetch_all(&self.conn).await;
        // Return empty if an error has occurred
        if r.is_err() { return vec![]; }
        // Return the unwrapped array of all
        // the class whitelisted users
        return r.unwrap();
    }

    // The delete_from_class_whitelist() function deletes 
    // an user from the provided class's whitelist. This
    // user can no longer access the provided class.
    pub async fn delete_from_class_whitelist(
        &self, class_hash: &str, data: Json<WhitelistDataBody>
    ) -> u64 {
        let r = sqlx::query!(
            "DELETE FROM whitelists WHERE class_hash=? AND whitelisted_user=?", class_hash, data.user
        ).execute(&self.conn).await;

        // If an error has occurred, return 0 rows affected
        if r.is_err() { return 0; }
        // Else, return the actual amount of rows that
        // have been affected by the insertion
        return r.unwrap().rows_affected();
    }

    // The insert_class_whitelist() function is used to add an
    // user into the provided class's whitelist. Users in this
    // whitelist can access the class info. The whitelist only
    // works if the teacher has enabled the class whitelist setting
    pub async fn insert_class_whitelist(
        &self, class_hash: &str, data: Json<WhitelistDataBody>
    ) -> u64 {
        let r = sqlx::query!(
            "INSERT INTO whitelists (class_hash, whitelisted_user) VALUES (?, ?)", 
            class_hash, data.user
        ).execute(&self.conn).await;

        // If an error has occurred, return 0 rows affected
        if r.is_err() { return 0; }
        // Else, return the actual amount of rows that
        // have been affected by the insertion
        return r.unwrap().rows_affected();
    }

    // The get_whitelist_json() function is used to
    // geterate a new json map as a string from the
    // provided whitelist array.
    pub fn get_whitelist_json(&self, whitelist: Vec<Whitelist>) -> String {
        // Define the json result string
        let mut r: String = String::new();
        // Iterate over the provided whitelisted users array
        // and append each of them to a formatted string array
        whitelist.iter().for_each(|f| {
            r.push_str(&format!("\"{}\",", f.whitelisted_user));
        });
        // Remove the last comma of the string array
        // before returning the new json map result
        return r[..r.len()-1].to_string();
    }
}