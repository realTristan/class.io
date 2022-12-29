use crate::lib::{
    self, structs::{
        Class, Announcement, ClassDataBody, Whitelist, Lesson, Unit
    }
};
use actix_web::web::Json;

// Database Implementation
impl lib::handlers::Database {
    // The insert_class_data() function is used to insert
    // a new class into the database. A maximum of
    // 5 classes is allowed per user. To generate the unique
    // class identifier, format the bearer with the current
    // time in nanoseconds.
    pub async fn insert_class_data(&self, bearer: &str, class_id: &str, class_name: &str) -> bool
    {
        // If the class already exists, return the function.
        if self.class_exists(class_id).await {
            return false;
        }

        // Get the bearer owner id
        let owner_id: String = match self.get_class_owner_id(bearer).await {
            Some(r) => r,
            None => return false
        };

        // Query the database
        let query = sqlx::query!(
            "INSERT INTO classes (owner_bearer, owner_id, class_id, class_name, enable_whitelist) VALUES (?, ?, ?, ?, ?)",
            bearer, owner_id, class_id, class_name, 0
        ).execute(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false
        };
    }

    // The get_class_owner_id() function is used to get
    // the user_id of the bearer token owner
    async fn get_class_owner_id(&self, bearer: &str) -> Option<String> 
    {
        // Query the database
        let query = sqlx::query!("SELECT user_id FROM users WHERE bearer=?", bearer)
            .fetch_one(&self.conn).await;
        
        // Return the user_id if not none
        return match query {
            Ok(r) => Some(r.user_id),
            Err(_) => None
        };
    }

    // The class_exists() function is used to check whether
    // the provided class hash already exists. This function
    // is called in the insert_class_data() function.
    async fn class_exists(&self, class_id: &str) -> bool 
    {
        // Query the database
        let query = sqlx::query!("SELECT * FROM classes WHERE class_id=?", class_id)
            .fetch_one(&self.conn).await;
        // Return whether valid query data has been obtained
        return !query.is_err();
    }

    // The get_class_update_query() function is used
    // to generate a string that will be used for updating
    // the class data within the database. This function
    // is required to disperse the query string from any
    // invalid/empty values.
    fn generate_class_update_query(&self, data: &Json<ClassDataBody>) -> String 
    {
        let mut res: String = String::new();
        // If the provided data's enable_whitelist integer bool
        // isn't invalid (equal to 2) then append the
        // updated value to the result string

        // FIX THIS FIND A WAY TO CHECK IF
        // VALUE IS INVALID NOT 2
        if data.enable_whitelist != 2 {
            // 2 == Invalid
            res.push_str(&format!("enable_whitelist={},", data.enable_whitelist));
        }
        // If the provided data's class_name length
        // is valid (greater than 0) then append the
        // updated value to the result string
        if data.class_name.len() > 0 {
            res.push_str(&format!("class_name='{}',", data.class_name));
        }
        // Remove the trailing comma at the end of the query
        return res[..res.len() - 1].to_string();
    }

    // The update_class_data() function is used to change
    // any data for the provided class within the database.
    // The function requires a generated class_update_query
    // which can be generated using the function above.
    pub async fn update_class_data(&self, bearer: &str, class_id: &str, data: &Json<ClassDataBody>) -> bool 
    {
        // Generate a new query string. This query string accounts
        // for empty values so that nothing gets corrupted.
        let query_data: String = self.generate_class_update_query(data);
        
        // Query the database
        let query = sqlx::query(&format!(
            "UPDATE classes SET {query_data} WHERE class_id='{class_id}' AND owner_bearer='{bearer}'"
        )).execute(&self.conn).await;
        
        // Return query result
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false,
        };
    }

    // The get_class_basic_data() function is used to get
    // all the primary class data. All the data names
    // are shown within the below comment.
    async fn get_class_basic_data(&self, class_id: &str) -> Option<Class> 
    {
        // Get the class primary data. This includes the class:
        // class_name, whitelist[bool], rls[bool], and class_id
        let query = sqlx::query_as!(
            Class,
            "SELECT class_name, owner_id, enable_whitelist FROM classes WHERE class_id=?",
            class_id
        ).fetch_one(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => Some(r),
            Err(_) => None,
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

    // The get_unit_lesson_json() function converts the
    // array of lessons into a readable json map that
    // will eventually be returned with the outgoing response body
    fn get_unit_lesson_json(&self, lessons: Vec<Lesson>) -> Vec<serde_json::Value>
    {
        let mut result: Vec<serde_json::Value> = Vec::new();

        // Iterate over the provided lessons array and
        // append each of the lesson's data to a formatted
        // string array of maps
        lessons.iter().for_each(|f| {
            result.push(serde_json::json!({
                "title": f.title,
                "description": f.description,
                "video": f.video,
                "work": f.work,
                "work_solutions": f.work_solutions
            }))
        });

        // Return the result array
        return result
    }

    // The get_unit_lessons() function is used to get all
    // the lesson data that comes with the provided unit hash.
    async fn get_unit_lessons(&self, unit_id: &str) -> Vec<Lesson>
    {
        // Query the database
        let query = sqlx::query_as!(
            Lesson,
            "SELECT title, description, video, work, work_solutions FROM lessons WHERE unit_id=?",
            unit_id
        ).fetch_all(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => r,
            Err(_) => Vec::new(),
        };
    }

    // The get_units_json() function is used to generate
    // a new json map as a string from the provided units array.
    pub async fn get_units_json(&self, units: Vec<Unit>) -> Vec<serde_json::Value>
    {
        let mut result: Vec<serde_json::Value> = Vec::new();

        // Iterate over the provided units array and
        // append each of the units data to a formatted
        // string array of maps
        for u in units {
            // Get the lessons that correspond with the unit
            let l: Vec<Lesson> = self.get_unit_lessons(&u.unit_id).await;

            // Append the unit json to the result string
            result.push(serde_json::json!({
                "unit_name": u.unit_name,
                "locked": u.locked == 1,
                "lessons": self.get_unit_lesson_json(l)
            }));
        }

        // Return the result array
        return result
    }

    // The get_class_data() function is used to get all data
    // revolving around the provided class_id. This includes
    // the class's primary data (shown below) and the class's
    // units and lessons.
    pub async fn get_class_data(&self, class_id: &str) -> Option<serde_json::Value>
    {
        let class = self.get_class_basic_data(class_id).await;
        // If the class doesn't exist, return an empty
        // json map. This is required before proceeding
        // with anything else to avoid errors
        if class.is_none() {
            return None
        }

        // If the class does exist, get all of it's data
        let units = self.get_class_units(class_id).await;
        let whitelist = self.get_class_whitelist(class_id).await;
        let announcements = self.get_class_announcements(class_id).await;

        // Else, unwrap the class data so that
        // it can be used in the response json
        let class: Class = class.unwrap();

        // Return a formatted string of all the class data
        return Some(serde_json::json!({
            "class_id": class_id,
            "owner_id": class.owner_id,
            "class_name": class.class_name,
            "enable_whitelist": class.enable_whitelist == 1,
            "units": self.get_units_json(units).await,
            "whitelist": self.get_whitelist_json(whitelist),
            "announcements": self.get_announcements_json(announcements)
        }));
    }
}
