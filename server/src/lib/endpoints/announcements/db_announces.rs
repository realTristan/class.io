use actix_web::web::Json;
use crate::lib::{self, global};
use super::endp_announces::AnnouncementDataBody;

// The Announcement data struct is used to
// store the announcement author's unique identifier,
// the authors name, the announcement title and description,
// along with any attachments the author has posted with it.
pub struct Announcement {
    // The announcement's author name
    // Use the get_user_data endpoint to get this
    author_name: String,
    // The announcement title
    title: String,
    // The announcements content
    description: String,
    // Any images/videos attached with the announcement
    attachment: String   // Base64 encode images, etc.
}

// Database Implementation
impl lib::handlers::Database {


    pub async fn insert_class_announcement(
        &self, class_hash: &str, data: Json<AnnouncementDataBody>
    ) -> u64 {
        let announcement_hash: String = global::generate_new_hash(class_hash);
        let time: i64 = global::get_time() as i64;
        
        let r = sqlx::query!(
            "INSERT INTO announcements (class_hash, announcement_hash, author_name, title, description, attachment, date)", 
            class_hash, announcement_hash, data.author_name, data.title, data.description, data.attachment, time
        ).execute(&self.conn).await;
        // If an error has occurred, return 0 rows affected
        if r.is_err() { return 0; }
        // Else, return the amount of affected rows
        return r.unwrap().rows_affected();
    }

    pub async fn delete_class_announcement(
        &self, announcement_hash: &str, data: Json<AnnouncementDataBody>
    ) -> u64 {
        let r = sqlx::query!(
            "DELETE FROM announcements WHERE announcement_hash=?", announcement_hash
        ).execute(&self.conn).await;
        // If an error has occurred, return 0 rows affected
        if r.is_err() { return 0; }
        // Else, return the amount of affected rows
        return r.unwrap().rows_affected();
    }

    // The get_class_announcements() function is used
    // to get all the announcements a teacher has
    // made within provided class_hash.
    pub async fn get_class_announcements(&self, class_hash: &str) -> Vec<Announcement> {
        // Fetch all the announcements that the
        // class owner has created.
        let r = sqlx::query_as!(
            Announcement, "SELECT author_name, title, description, attachment FROM announcements WHERE class_hash=?", class_hash
        ).fetch_all(&self.conn).await;
        // Return empty if an error has occurred
        if r.is_err() { return vec![]; }
        // Return the unwrapped array of all
        // the class announcements
        return r.unwrap();
    }

    // The get_announcements_json() function is used to
    // generate a new json map as a string from the
    // provided announcements array.
    pub fn get_announcements_json(&self, announcements: Vec<Announcement>) -> String {
        // Define the json result string
        let mut r: String = String::new();
        // Iterate over the provided announcements array and
        // append each of the announcement's data to a formatted
        // string array of maps
        announcements.iter().for_each(|f| {
            r.push_str(
                &format!("{{
                    \"author_name\": \"{}\", 
                    \"title\": \"{}\", 
                    \"description\": \"{}\", 
                    \"attachment\": \"{}\"
                }},", 
                f.author_name, f.title, f.description, f.attachment
            ))
        });
        // Remove the last comma of the string array
        // before returning the new json map result
        return r[..r.len()-1].to_string();
    }
}