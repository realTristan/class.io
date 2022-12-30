use crate::lib::{
    self, structs::{
        Class, Announcement, Whitelist, Unit, Lesson
    }
};

// Database Implementation
impl lib::handlers::Database {
    // The class_exists() function is used to check whether
    // the provided class hash already exists. This function
    // is called in the insert_class_data() function.
    async fn class_exists(&self, class_id: &str) -> bool 
    {
        // Query the database
        let query = sqlx::query!(
            "SELECT * FROM classes WHERE class_id=?", class_id
        ).fetch_one(&self.conn).await;
        
        // Return whether valid query data has been obtained
        return !query.is_err();
    }

    // The insert_class_data() function is used to insert
    // a new class into the database. A maximum of
    // 5 classes is allowed per user. To generate the unique
    // class identifier, format the bearer with the current
    // time in nanoseconds.
    pub async fn insert_class_data(
        &self, bearer: &str, class_id: &str, class_name: &str
    ) -> bool {

        // If the class already exists, return the function.
        if self.class_exists(class_id).await {
            return false;
        }

        // Get the bearer owner id
        let owner_id: String = match self.get_user_id_by_bearer(bearer).await {
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


    // The get_class_update_query() function is used
    // to generate a string that will be used for updating
    // the class data within the database. This function
    // is required to disperse the query string from any
    // invalid/empty values.
    fn generate_class_update_query(&self, data: serde_json::Value) -> String 
    {
        // Create a new string
        let mut query: String = String::new();

        // Get the enable whitelist bool from the request body
        match data["enable_whitelist"].as_bool() {
            Some(r) => query.push_str(&format!("enable_whitelist={},", r as i8)),
            None => ()
        };
        // Get the class name from the request body
        match data["class_name"].as_str() {
            Some(name) => query.push_str(&format!("class_name='{}',", name)),
            None => ()
        };

        // Remove the trailing comma at the end of the query
        return query[..query.len() - 1].to_string();
    }

    // The update_class_data() function is used to change
    // any data for the provided class within the database.
    // The function requires a generated class_update_query
    // which can be generated using the function above.
    pub async fn update_class_data(
        &self, bearer: &str, class_id: &str, data: serde_json::Value
    ) -> bool {

        // Generate a new query string. This query string accounts
        // for empty values so that nothing gets corrupted.
        let query_data: String = self.generate_class_update_query(data);
        
        // Query the database
        let query = sqlx::query(&format!(
            "UPDATE classes SET {} WHERE class_id='{}' AND owner_bearer='{}'", 
            query_data, class_id, bearer
        )).execute(&self.conn).await;
        
        // Return query result
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false,
        };
    }


    // The get_class_data() function is used to get all data
    // revolving around the provided class_id. This includes
    // the class's primary data (shown below) and the class's
    // units and lessons.
    pub async fn get_class_data(&self, class_id: &str) -> Option<serde_json::Value>
    {
        // Get the class's general data
        let class: Class = match self.get_class_general_data(class_id).await {
            Some(r) => r,
            None => return None
        };

        // If the class does exist, get all of it's data
        let units = self.get_class_units(class_id).await;
        let whitelist = self.get_class_whitelist(class_id).await;
        let announcements = self.get_class_announcements(class_id).await;

        // Return a formatted string of all the class data
        return Some(serde_json::json!({
            "class_id": class_id,
            "owner_id": class.owner_id,
            "class_name": class.class_name,
            "enable_whitelist": class.enable_whitelist == 1,
            "units": units,
            "whitelist": whitelist,
            "announcements": announcements
        }));
    }

    // The get_class_general_data() function is used to get
    // all the primary class data. All the data names
    // are shown within the below comment.
    async fn get_class_general_data(&self, class_id: &str) -> Option<Class> 
    {
        // Get the class's general data. This includes the class:
        // class_name, whitelist[bool], rls[bool], and class_id
        let query = sqlx::query_as!(Class,
            "SELECT class_name, owner_id, enable_whitelist FROM classes WHERE class_id=?",
            class_id
        ).fetch_one(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => Some(r),
            Err(_) => None,
        };
    }

    // The get_class_units() function is used to
    // easily get all the units corresponding with
    // the provided class_id.
    pub async fn get_class_units(&self, class_id: &str) -> Vec<serde_json::Value>
    {
        // Query the database
        let query = sqlx::query_as!(Unit,
            "SELECT unit_id, unit_name, locked FROM units WHERE class_id=?",
            class_id
        ).fetch_all(&self.conn).await;


        // Return the result of the query
        return match query {
            Err(_) => Vec::new(),
            Ok(r) => futures::future::join_all(r.iter().map(|u| async {
                serde_json::json!({
                    "unit_name": u.unit_name,
                    "locked": u.locked == 1,
                    "lessons": self.get_unit_lessons(&u.unit_id).await
                })
            })).await
        }
    }
    
    // The get_unit_lessons() function is used to get all
    // the lesson data that comes with the provided unit hash.
    // The function then converts the query data into a readable 
    // json map that will eventually be returned with the 
    // outgoing response body
    async fn get_unit_lessons(&self, unit_id: &str) -> Vec<serde_json::Value>
    {
        // Query the database
        let query = sqlx::query_as!(Lesson,
            "SELECT title, description, video, work, work_solutions FROM lessons WHERE unit_id=?",
            unit_id
        ).fetch_all(&self.conn).await;

        // Return query result
        return match query {
            Err(_) => Vec::new(),
            Ok(r) => r.iter().map(|f| {
                serde_json::json!({
                    "title": f.title,
                    "description": f.description,
                    "video": f.video,
                    "work": f.work,
                    "work_solutions": f.work_solutions
                })
            }).collect()
        };
    }


    // The get_class_announcements() function is used
    // to get all the announcements a teacher has
    // made within provided class_id.
    pub async fn get_class_announcements(&self, class_id: &str) -> Vec<serde_json::Value>
    {
        // Fetch all the announcements that the
        // class owner has created.
        let query = sqlx::query_as!(Announcement, 
            "SELECT author_name, title, description, attachment FROM announcements WHERE class_id=?", 
            class_id
        ).fetch_all(&self.conn).await;

        // Return query result
        return match query {
            Err(_) => Vec::new(),
            Ok(r) => r.iter().map(|f| {
                serde_json::json!({
                    "author_name": f.author_name,
                    "title": f.title,
                    "description": f.description,
                    "attachment": f.attachment
                })
            }).collect()
        };
    }

    // The get_class_whitelist() function is used to return
    // an array containing all the users that are allowed to
    // see the content within the provided class_id
    pub async fn get_class_whitelist(&self, class_id: &str) -> Vec<serde_json::Value>
    {
        // Fetch all the whitelisted users that have
        // access to the provided class.
        let query = sqlx::query_as!(Whitelist, 
            "SELECT whitelisted_user_name, whitelisted_user_id FROM whitelists WHERE class_id=?", 
            class_id
        ).fetch_all(&self.conn).await;

        // Return the result of the query
        return match query {
            Err(_) => Vec::new(),
            Ok(r) => r.iter().map(|f| {
                serde_json::json!({
                    "whitelisted_user_name": f.whitelisted_user_name,
                    "whitelisted_user_id": f.whitelisted_user_id
                })
            }).collect()
        }
    }
}
