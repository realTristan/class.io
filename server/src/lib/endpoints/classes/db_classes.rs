use super::endp_classes::ClassDataBody;
use actix_web::web::Json;
use crate::lib;

// The Class data struct is used to store
// the classes owner_hash, unique class identifier,
// class name, class whitelist array, class announcements,
// rsl bool, and the class units.
pub struct Class {
    // The Teacher's unique user identifier
    pub owner_hash: String,
    // The Classes unique identifier
    pub class_hash: String,
    // The Class Name
    pub class_name: String,
    // Whether the students need to be logged in to
    // access this class
    pub rsl: i64
}
// The Announcement data struct is used to
// store the announcement author's unique identifier,
// the authors name, the announcement title and description,
// along with any attachements the author has posted with it.
struct Announcement {
    // The author's unique identifier (required for bearer token)
    author_hash: String,
    // The announcement's author name
    // Use the get_user_data endpoint to get this
    author_name: String,
    // The announcement title
    title: String,
    // The announcements content
    description: String,
    // Any images/videos attached with the announcement
    attachement: String   // Base64 encode images, etc.
}
// The Unit data struct is used to store
// the class unit's unique identifier, 
// unit name, it's locked status and the
// lessons that come along with the unit.
pub struct Unit {
    // The unique unit identifier
    pub unit_hash: String,
    // The Unit's Name
    pub unit_name: String,
    // Whether students can access this unit yet
    pub locked: i64
}
// The Lesson data struct is used to store
// the class unit's lesson title, description,
// video_url, work and work_solutions.
struct Lesson {
    // The Lesson Title
    title: String,
    // The Lesson Description
    description: String,
    // The Lesson's Youtube Video URL
    video: String,
    // The Lesson Homework that can be 
    // submitted and marked
    work: String,
    // The Lesson Homework Solutions
    work_solutions: String
}
// The Whitelist data struct is used for querying
// the whitelisted users for a specific class.
struct Whitelist { whitelisted_user: String }

// Database Implementation
impl lib::database::Database {
    // The generate_class_update_query() function is used
    // to generate a string that will be used for updating
    // the class data within the database. This function
    // is required to disperse the query string from any
    // invalid/empty values.
    fn generate_class_update_query(&self, data: Json<ClassDataBody>) -> String {
        let mut res: String = String::new();
        // If the provided data's enable_whitelist integer bool
        // isn't invalid (equal to 2) then append the
        // updated value to the result string
        if data.enable_whitelist != 2 { // 2 == Invalid
            res.push_str(&format!("enable_whitelist={}", data.enable_whitelist));
        }
        // If the provided data's rsl integer bool
        // isn't invalid (equal to 2) then append the
        // updated value to the result string
        if data.rsl != 2 { // 2 == Invalid
            res.push_str(&format!("rsl={}", data.rsl));
        }
        // If the provided data's class_name length
        // is valid (greater than 0) then append the
        // updated value to the result string
        if data.class_name.len() > 0 {
            res.push_str(&format!("class_name={}", data.class_name));
        }
        return res
    }

    // The update_class_data() function is used to change
    // any data for the provided class within the database.
    // The function requires a generated class_update_query
    // which can be generated using the function above.
    pub async fn update_class_data(&self, class_hash: &str, data: Json<ClassDataBody>) -> u64 {
        let q: String = self.generate_class_update_query(data);
        return sqlx::query(
            &format!("UPDATE classes SET {q} WHERE class_hash={class_hash}"))
                .execute(&self.conn).await.unwrap().rows_affected();
    }

    // The get_class_prime_data() function is used to get
    // all the primary class data. All the data names
    // are shown within the below comment.
    async fn get_class_prime_data(&self, class_hash: &str) -> Option<Class> {
        // Get the class primary data. This includes the class:
        // class_name, whitelist[bool], rls[bool], 
        // owner_hash, and class_hash
        let r = sqlx::query_as!(
            Class, "SELECT owner_hash, class_hash, class_name, rsl FROM classes WHERE class_hash=?", class_hash
        ).fetch_one(&self.conn).await;
        // Return empty if an error has occurred
        if r.is_err() { return None; }
        // Else, if no error has occured, return
        // the queried class data
        return Some(r.unwrap());
    }

    // The get_class_units() function is used to
    // easily get all the units corresponding with
    // the provided class_hash.
    async fn get_class_units(&self, class_hash: &str) -> Vec<Unit> {
        // Fetch all the units that are in the class.
        // By the end of the function, all the unit data
        // will be neatly categorized.
        let r = sqlx::query_as!(
            Unit, "SELECT unit_hash, unit_name, locked FROM units WHERE class_hash=?", class_hash
        ).fetch_all(&self.conn).await;
        // Return empty if an error has occurred
        if r.is_err() { return vec![]; }
        // Else if no error has occurred, return
        // the unwrapped array of all the units
        return r.unwrap();
    }

    // The get_unit_lessons() function is used to get
    // all the lesson data that comes with the provided
    // unit hash.
    async fn get_unit_lessons(&self, unit_hash: &str) -> Vec<Lesson> {
        let r = sqlx::query_as!(
            Lesson, "SELECT title, description, video, work, work_solutions FROM lessons WHERE unit_hash=?", unit_hash
        ).fetch_all(&self.conn).await;
        // Return empty if an error has occurred
        if r.is_err() { return vec![]; }
        // Else if no error has occurred, return
        // the unwrapped array of all the units
        return r.unwrap();
    }

    // The get_class_announcements() function is used
    // to get all the announcements a teacher has
    // made within provided class_hash.
    async fn get_class_announcements(&self, class_hash: &str) -> Vec<Announcement> {
        // Fetch all the announcements that the
        // class owner has created.
        let r = sqlx::query_as!(
            Announcement, "SELECT author_hash, author_name, title, description, attachement FROM announcements WHERE class_hash=?", class_hash
        ).fetch_all(&self.conn).await;
        // Return empty if an error has occurred
        if r.is_err() { return vec![]; }
        // Return the unwrapped array of all
        // the class announcements
        return r.unwrap();
    }

    // The get_class_whitelist() function is used to return
    // an array containing all the users that are allowed to
    // see the content within the provided class_hash
    async fn get_class_whitelist(&self, class_hash: &str) -> Vec<Whitelist> {
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

    // The get_class_data() function is used to get all data
    // revolving around the provided class_hash. This includes
    // the class's primary data (shown below) and the class's
    // units and lessons.
    pub async fn get_class_data(&self, class_hash: &str) -> &str {
        let prime_data: Option<Class> = self.get_class_prime_data(class_hash).await;
        let units: Vec<Unit> = self.get_class_units(class_hash).await;
        let whitelist: Vec<Whitelist> = self.get_class_whitelist(class_hash).await;
        let announcements: Vec<Announcement> = self.get_class_announcements(class_hash).await;

        // Return an empty json map
        if prime_data.is_none() { return "{}" }
        // Return a json map with all the above data
        return "{}"
    }
}