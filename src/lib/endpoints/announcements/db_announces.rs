use super::endp_announces::AnnouncementDataBody;
use crate::lib::{self, global};
use actix_web::web::Json;

// The Announcement data struct is used to
// store the announcement author's unique identifier,
// the authors name, the announcement title and description,
// along with any attachments the author has posted with it.
pub struct Announcement {
    // The announcement's author name
    author_name: String,
    // The announcement title
    title: String,
    // The announcements content
    description: String,
    // Any images/videos attached with the announcement
    attachment: String, // Base64 encode images, etc.
}

// Database Implementation
impl lib::handlers::Database {
    // The insert_class_announcement() function is used to
    // create a new announcement for the provided class_id.
    // A unique announcement identifier is created before hand
    // so that if the announcement author wants to delete
    // their announcement, they can. Along with this, a post
    // date is also inserted into the database.
    pub async fn insert_class_announcement(
        &self, bearer: &str, class_id: &str, announcement_id: &str, data: &Json<AnnouncementDataBody>
    ) -> bool {
        // If the announcement hash already exists, return the function
        if self.class_announcement_exists(announcement_id).await {
            return false;
        }

        // Get the current date of the announcement post
        let date: i64 = global::get_time().as_secs() as i64;

        // Query the database, inserting the new announcement
        // along with all of it's data.
        let query = sqlx::query!(
            "INSERT INTO announcements (owner_bearer, class_id, announcement_id, author_name, title, description, attachment, date) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", 
            bearer, class_id, announcement_id, data.author_name, data.title, data.description, data.attachment, date
        ).execute(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false,
        };
    }

    // The class_announcement_exists() function is used to check whether
    // the provided announcement hash already exists. This function
    // is called in the insert_class_announcement() function.
    async fn class_announcement_exists(&self, announcement_id: &str) -> bool 
    {
        // Query the database
        let query = sqlx::query!(
            "SELECT * FROM announcements WHERE announcement_id=?",
            announcement_id
        ).fetch_one(&self.conn).await;

        // Return whether valid query data has been obtained
        return !query.is_err();
    }

    // The delete_class_announcement() function is used
    // to delete a specific announcement post using
    // the provided announcement_id.
    pub async fn delete_class_announcement(&self, bearer: &str, announcement_id: &str) -> bool 
    {
        // Query the database, deleting the announcement with
        // the incoming requests data.announcement_id
        let query = sqlx::query!(
            "DELETE FROM announcements WHERE announcement_id=? AND owner_bearer=?",
            announcement_id, bearer
        ).execute(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false
        };
    }

    // The get_class_announcements() function is used
    // to get all the announcements a teacher has
    // made within provided class_id.
    pub async fn get_class_announcements(&self, class_id: &str) -> Vec<Announcement> 
    {
        // Fetch all the announcements that the
        // class owner has created.
        let query = sqlx::query_as!(
            Announcement, "SELECT author_name, title, description, attachment FROM announcements WHERE class_id=?", 
            class_id
        ).fetch_all(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => r,
            Err(_) => Vec::new()
        };
    }

    // The get_announcements_json() function is used to
    // generate a new json map as a string from the
    // provided announcements array.
    pub fn get_announcements_json(&self, announcements: Vec<Announcement>) -> Vec<serde_json::Value> 
    {
        let mut result: Vec<serde_json::Value> = Vec::new();

        // Iterate over the provided announcements array and
        // append each of the announcement's data to a formatted
        // string array of maps
        announcements.iter().for_each(|f| {
            result.push(serde_json::json!({
                "author_name": f.author_name,
                "title": f.title,
                "description": f.description,
                "attachment": f.attachment
            }))
        });
        
        // Return the result array
        return result
    }
}
