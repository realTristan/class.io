use super::endp_class::ClassDataBody;
use actix_web::web::Json;
use crate::lib;

// The Class data struct is used to store
// the classes owner_hash, unique class identifier,
// class name, class whitelist array, class announcements,
// rsl bool, and the class units.
pub struct Class {
    // The Class Name
    class_name: String,
    // Whether the students need to be logged in to
    // access this class
    rsl: i64,
    // Whether to the use the class whitelist
    enable_whitelist: i64,
}

// Database Implementation
impl lib::handlers::Database {
    // The insert_test_class() function is used for endpoint
    // debugging as it is required that atleast one class be
    // present in order to properly test.
    pub async fn insert_test_class(&self) {
        println!("Test User Hash: 22f3d5b9c91b570a4f1848c5d147b4709d2fb96");
        println!("Test Class Hash: e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020");

        // Insert into CLASSES column
        sqlx::query!(
            "INSERT INTO classes (owner_hash, class_hash, class_name, rsl, enable_whitelist) VALUES (?, ?, ?, ?, ?)",
            "822f3d5b9c91b570a4f1848c5d147b4709d2fb96", "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "Advanced Functions", 0, 0
        ).execute(&self.conn).await.unwrap();

        // Insert into ANNOUNCEMENTS column
        sqlx::query!(
            "INSERT INTO announcements (class_hash, announcement_hash, author_name, title, description, attachment, date) VALUES (?, ?, ?, ?, ?, ?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "announcement_hash", "Tristan Simpson", "Test Announcement", "Hey guys!", "no_attachment", 0
        ).execute(&self.conn).await.unwrap();
        
        // Insert into WHITELISTS column
        sqlx::query!(
            "INSERT INTO whitelists (class_hash, whitelisted_user) VALUES (?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "test_whitelisted_user1"
        ).execute(&self.conn).await.unwrap();

        // Insert into LESSONS column
        sqlx::query!(
            "INSERT INTO lessons (unit_hash, title, description, video, work, work_solutions) VALUES (?, ?, ?, ?, ?, ?)",
            "random_unit_hash", "test_lesson_title", "test_lesson_desc", "test_lesson_video", "test_lesson_work", "test_lesson_work_solutions"
        ).execute(&self.conn).await.unwrap();

        // Insert into UNITS column
        sqlx::query!(
            "INSERT INTO units (class_hash, unit_hash, unit_name, locked) VALUES (?, ?, ?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "random_unit_hash", "Polynomials", 0
        ).execute(&self.conn).await.unwrap();

        // Insert into UNITS column
        sqlx::query!(
            "INSERT INTO units (class_hash, unit_hash, unit_name, locked) VALUES (?, ?, ?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "random_unit_hash", "Functions", 0
        ).execute(&self.conn).await.unwrap();

        // Insert into UNITS column
        sqlx::query!(
            "INSERT INTO units (class_hash, unit_hash, unit_name, locked) VALUES (?, ?, ?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "random_unit_hash", "Calculus", 0
        ).execute(&self.conn).await.unwrap();

        // Insert into SUBMISSIONS column
        sqlx::query!(
            "INSERT INTO submissions (class_hash, submission_hash, submitter_hash, submission_date, data) VALUES (?, ?, ?, ?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "submission_hash", "822f3d5b9c91b570a4f1848c5d147b4709d2fb96", 0, ""
        ).execute(&self.conn).await.unwrap();
    }

    // The insert_class_data() function is used to insert
    // a new class into the database. A maximum of
    // 5 classes is allowed per user. To generate the unique
    // class identifier, format the user_hash with the current
    // time in nanoseconds.
    pub async fn insert_class_data(
        &self, user_hash: &str, class_hash: &str, class_name: &str
    ) -> u64 {
        // If the class already exists, return the function.
        if self.class_exists(class_hash).await { return 0; }

        // Query the database
        let r = sqlx::query!(
            "INSERT INTO classes (owner_hash, class_hash, class_name, rsl, enable_whitelist) VALUES (?, ?, ?, ?, ?)",
            user_hash, class_hash, class_name, 0, 0
        ).execute(&self.conn).await;

        // If an error has occurred, return 0 rows affected
        if r.is_err() { return 0; }
        // Else, return the amount of affected rows
        return r.unwrap().rows_affected();
    }

    // The class_exists() function is used to check whether
    // the provided class hash already exists. This function
    // is called in the insert_class_data() function.
    async fn class_exists(&self, class_hash: &str) -> bool {
        // Query the database
        let r = sqlx::query!(
            "SELECT * FROM classes WHERE class_hash=?", class_hash
        ).fetch_one(&self.conn).await;
        // Return whether valid query data has been obtained
        return !r.is_err();
    }

    // The get_class_update_query() function is used
    // to generate a string that will be used for updating
    // the class data within the database. This function
    // is required to disperse the query string from any
    // invalid/empty values.
    fn generate_class_update_query(&self, data: &Json<ClassDataBody>) -> String {
        let mut res: String = String::new();
        // If the provided data's enable_whitelist integer bool
        // isn't invalid (equal to 2) then append the
        // updated value to the result string

        // FIX THIS FIND A WAY TO CHECK IF
        // VALUE IS INVALID NOT 2
        if data.enable_whitelist != 2 { // 2 == Invalid
            res.push_str(&format!("enable_whitelist={},", data.enable_whitelist));
        }
        // If the provided data's rsl integer bool
        // isn't invalid (equal to 2) then append the
        // updated value to the result string
        if data.rsl != 2 { // 2 == Invalid
            res.push_str(&format!("rsl={},", data.rsl));
        }
        // If the provided data's class_name length
        // is valid (greater than 0) then append the
        // updated value to the result string
        if data.class_name.len() > 0 {
            res.push_str(&format!("class_name='{}',", data.class_name));
        }
        // Remove the trailing comma at the end of the query
        return res[..res.len()-1].to_string()
    }

    // The update_class_data() function is used to change
    // any data for the provided class within the database.
    // The function requires a generated class_update_query
    // which can be generated using the function above.
    pub async fn update_class_data(
        &self, user_hash: &str, class_hash: &str, data: &Json<ClassDataBody>
    ) -> u64 {
        // Generate a new query string. This query string accounts
        // for empty values so that nothing gets corrupted.
        let q: String = self.generate_class_update_query(data);
        // Query the database
        let r = sqlx::query(
            &format!("UPDATE classes SET {q} WHERE class_hash='{class_hash}' AND owner_hash='{user_hash}'"))
                .execute(&self.conn).await;
        // If an error has occurred, return 0 rows affected
        if r.is_err() { return 0; }
        // Else, return the amount of affected rows
        return r.unwrap().rows_affected();
    }

    // The get_class_basic_data() function is used to get
    // all the primary class data. All the data names
    // are shown within the below comment.
    async fn get_class_basic_data(&self, class_hash: &str) -> Option<Class> {
        // Get the class primary data. This includes the class:
        // class_name, whitelist[bool], rls[bool], and class_hash
        let r = sqlx::query_as!(
            Class, "SELECT class_name, rsl, enable_whitelist FROM classes WHERE class_hash=?", class_hash
        ).fetch_one(&self.conn).await;
        // Return empty if an error has occurred
        if r.is_err() { return None; }
        // Else, if no error has occured, return
        // the queried class data
        return Some(r.unwrap());
    }

    // The get_class_data() function is used to get all data
    // revolving around the provided class_hash. This includes
    // the class's primary data (shown below) and the class's
    // units and lessons.
    pub async fn get_class_data(&self, class_hash: &str) -> String {
        let class = self.get_class_basic_data(class_hash).await;
        let units = self.get_class_units(class_hash).await;
        let whitelist = self.get_class_whitelist(class_hash).await;
        let announcements = self.get_class_announcements(class_hash).await;
        // If the class is non existent, then return
        // an empty json map. This is required before
        // proceeding with anything else to avoid errors
        if class.is_none() { return "{}".to_string() }
        // Else, unwrap the class data so that
        // it can be used in the response json
        let class: Class = class.unwrap();

        // Return a formatted string of all the class data
        return format!(
            "{{
                \"class_hash\": \"{}\", 
                \"class_name\": \"{}\", 
                \"enable_whitelist\":{}, 
                \"rsl\":{}, 
                \"units\": [{}], 
                \"whitelist\": [{}], 
                \"announcements\": [{}]
            }}", 
            class_hash, class.class_name, class.enable_whitelist==1, class.rsl==1, 
            self.get_units_json(units).await, self.get_whitelist_json(whitelist), self.get_announcements_json(announcements),
        );
    }
}