use super::{endp_class::ClassDataBody, endp_unit::UnitDataBody, endp_whitelist::WhitelistDataBody, endp_submission::SubmissionDataBody};
use actix_web::web::Json;
use crate::lib;

// The Class data struct is used to store
// the classes owner_hash, unique class identifier,
// class name, class whitelist array, class announcements,
// rsl bool, and the class units.
pub struct Class {
    // The Classes unique identifier
    pub class_hash: String,
    // The Class Name
    pub class_name: String,
    // Whether the students need to be logged in to
    // access this class
    pub rsl: i64,
    // Whether to the use the class whitelist
    pub enable_whitelist: i64,
}
// The Announcement data struct is used to
// store the announcement author's unique identifier,
// the authors name, the announcement title and description,
// along with any attachments the author has posted with it.
struct Announcement {
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
// The Submission data struct is used to store
// the work: submitter_hash, submissions_hash,
// submission_date and work data
struct Submission {
    // The user who submitted the work's unique identifier
    submitter_hash: String,
    // The unique identifier of the submission
    submission_hash: String,
    // The date the work was submitted
    submission_date: i64,
    // The submission data. (ex: the file, the answers, etc.)
    data: String
}
// The Whitelist data struct is used for querying
// the whitelisted users for a specific class.
struct Whitelist { whitelisted_user: String }

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
            "822f3d5b9c91b570a4f1848c5d147b4709d2fb96", "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "test_class_name", 0, 0
        ).execute(&self.conn).await.unwrap();

        // Insert into ANNOUNCEMENTS column
        sqlx::query!(
            "INSERT INTO announcements (class_hash, author_name, title, description, attachment, date) VALUES (?, ?, ?, ?, ?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "test_author_name", "test_title", "test_desc", "no_attachment", 0
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
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "random_unit_hash", "test_unit_name", 0
        ).execute(&self.conn).await.unwrap();

        // Insert into SUBMISSIONS column
        sqlx::query!(
            "INSERT INTO submissions (class_hash, submitter_hash, submission_date, data) VALUES (?, ?, ?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "822f3d5b9c91b570a4f1848c5d147b4709d2fb96", 0, ""
        ).execute(&self.conn).await.unwrap();
    }

    // The generate_new_hash() function is used to generate
    // a unique hash using the provided identifier (class_hash, user_hash, etc.)
    // and the current time in nanoseconds.
    fn generate_new_hash(&self, identifier: &str) -> String {
        // Get the current time since epoch. This duration is later converted
        // into nanoseconds to ensure that the class hash is 100% unique.
        let time: std::time::Duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap();
        // Generate a new hash using the provided
        // class hash, and the current time as nanoseconds.
        return format!("{}:{}", identifier, time.as_nanos());
    }
    
    // The insert_class_submission() function is used to
    // insert a new work submission into the database
    // using the provided class hash. The function generates
    // a unique submission hash before inserting the data, which
    // is used within the delete_class_submission() function
    pub async fn insert_class_submission(&self, class_hash: &str, data: Json<SubmissionDataBody>) -> u64 {
        // Get the current time since epoch. This duration is later converted
        // into nanoseconds to ensure that the class hash is 100% unique.
        let time: std::time::Duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap();
        // Generate a new hash using the provided
        // class hash, and the current time as nanoseconds.
        let submission_hash: String = format!("{}:{}", data.submitter_hash, time.as_nanos());
        let date: i64 = time.as_secs() as i64;

        // Insert the data into the database
        let r = sqlx::query!(
            "INSERT INTO submissions (class_hash, submission_hash, submitter_hash, submission_date, data) VALUES (?, ?, ?, ?, ?)", 
            class_hash, submission_hash, data.submitter_hash, date, data.data
        ).execute(&self.conn).await;

        // If an error has occurred, return 0 rows affected
        if r.is_err() { return 0; }
        // Else, return the actual amount of rows that
        // have been affected by the insertion
        return r.unwrap().rows_affected();
    }

    // The delete_class_submission() function is used to
    // delete a submission from the database. This function
    // is called when a student wants to unsubmit a portion
    // of their work.
    pub async fn delete_class_submission(&self, data: Json<SubmissionDataBody>) -> u64 {
        // Query the database, deleting all data revolving around
        // the provided submission hash
        let r = sqlx::query!(
            "DELETE FROM submissions WHERE submission_hash=?", data.submission_hash
        ).execute(&self.conn).await;

        // If an error has occurred, return 0 rows affected
        if r.is_err() { return 0; }
        // Else, return the actual amount of rows that
        // have been affected by the insertion
        return r.unwrap().rows_affected();
    }

    // The get_class_submissions() function is used to 
    // return all the submissions for the provided class.
    // This function is used in the dashboard of the website
    // where the teachers can mark the students submitted work.
    pub async fn get_class_submissions(&self, class_hash: &str) -> String {
        // Query the database, selecting the submitter_hash, submission_date
        // and the submission data from the submissions column
        let r = sqlx::query_as!(
            Submission, "SELECT submitter_hash, submission_hash, submission_date, data FROM submissions WHERE class_hash=?", 
            class_hash
        ).fetch_all(&self.conn).await;

        // Return empty if an error has occurred
        if r.is_err() { return "{}".to_string() }
        // Else if no error has occurred, return
        // the unwrapped array of all the units
        return format!("[{}]", self.get_submission_json(r.unwrap()));
    }

    // The get_user_submissions() function is used to get all the
    // submissions from the provided user_hash within the provided
    // class. This function is used for the students to see
    // what work they have previously submitted. This function is the
    // basis towards students being able to insert and delete submissions.
    pub async fn get_user_submissions(&self, class_hash: &str, user_hash: &str) -> String {
        // Query the database selecting the submitter_hash, submission_hash, submission_date
        // and the submission data from the submissions column.
        let r = sqlx::query_as!(
            Submission, "SELECT submitter_hash, submission_hash, submission_date, data 
                            FROM submissions WHERE class_hash=? AND submitter_hash=?", 
            class_hash, user_hash
        ).fetch_all(&self.conn).await;

        // Return empty if an error has occurred
        if r.is_err() { return "{}".to_string() }
        // Else if no error has occurred, return
        // the unwrapped array of all the units
        return format!("[{}]", self.get_submission_json(r.unwrap()));
    }

    // The insert_class_unit() function is used to insert a new
    // unit into the database for the provided class. Students who
    // visit the class through the website, will see this unit appear.
    pub async fn insert_class_unit(&self, class_hash: &str, data: Json<UnitDataBody>) -> u64 {
        // Generate a new unit hash using the provided class_hash
        let unit_hash: String = self.generate_new_hash(class_hash);

        // Insert the data into the database
        let r = sqlx::query!(
            "INSERT INTO units (class_hash, unit_hash, unit_name, locked) VALUES (?, ?, ?, ?)", 
            class_hash, unit_hash, data.unit_name, 0
        ).execute(&self.conn).await;

        // If an error has occurred, return 0 rows affected
        if r.is_err() { return 0; }
        // Else, return the actual amount of rows that
        // have been affected by the insertion
        return r.unwrap().rows_affected();
    }

    // The delete_class_unit() function is used to delete a unit
    // from the units column wherever the provided unit_hash
    // is present. A maximum of 12 units is allowed per class.
    pub async fn delete_class_unit(&self, data: Json<UnitDataBody>) -> u64 {
        let r = sqlx::query!(
            "DELETE FROM units WHERE unit_hash=?", data.unit_hash
        ).execute(&self.conn).await;

        // If an error has occurred, return 0 rows affected
        if r.is_err() { return 0; }
        // Else, return the actual amount of rows that
        // have been affected by the insertion
        return r.unwrap().rows_affected();
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

    // The update_class_unit() function is used to update a class's
    // unit data replacing the current data, with that of the provided
    // Json<UnitDataBody> values. In order to prevent null values
    // being updated, the function first determines which values are null.
    pub async fn update_class_unit(&self, data: Json<UnitDataBody>) -> u64 {
        let mut res: String = String::new();
        // If the provided data's enable_whitelist integer bool
        // isn't invalid (equal to 2) then append the
        // updated value to the result string
        if data.unit_name.len() > 0 {
            res.push_str(&format!("unit_name='{}',", data.unit_name));
        }
        // If the provided data's rsl integer bool
        // isn't invalid (equal to 2) then append the
        // updated value to the result string
        if data.locked != 2 { // 2 == Invalid
            res.push_str(&format!("locked={},", data.locked));
        }
        // Remove the trailing comma
        let res: String = res[..res.len()-1].to_string();

        // Query the database, updating all the values
        // in the above res: String that have the same
        // unit_hash as the one provided
        let r = sqlx::query(
            &format!("UPDATE units SET {} unit_hash='{}'", res, data.unit_hash)
        ).execute(&self.conn).await;

        if r.is_err() { return 0; }
        return r.unwrap().rows_affected();
    }

    // The insert_class_data() function is used to insert
    // a new class into the database. A maximum of
    // 5 classes is allowed per user. To generate the unique
    // class identifier, format the user_hash with the current
    // time in nanoseconds.
    pub async fn insert_class_data(&self, data: Json<ClassDataBody>) -> u64 {
        // Generate a new class hash using the provided class_hash
        let class_hash: String = self.generate_new_hash(&data.user_hash);
        // Query the database
        let r = sqlx::query!(
            "INSERT INTO classes (owner_hash, class_hash, class_name, rsl, enable_whitelist) VALUES (?, ?, ?, ?, ?)",
            data.user_hash, class_hash, data.class_name, 0, 0
        ).execute(&self.conn).await;

        // If an error has occurred, return 0 rows affected
        if r.is_err() { return 0; }
        // Else, return the amount of affected rows
        return r.unwrap().rows_affected();
    }

    // The get_class_update_query() function is used
    // to generate a string that will be used for updating
    // the class data within the database. This function
    // is required to disperse the query string from any
    // invalid/empty values.
    fn generate_class_update_query(&self, data: Json<ClassDataBody>) -> String {
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
    pub async fn update_class_data(&self, class_hash: &str, data: Json<ClassDataBody>) -> u64 {
        // Generate a new query string. This query string accounts
        // for empty values so that nothing gets corrupted.
        let q: String = self.generate_class_update_query(data);
        // Query the database
        let r = sqlx::query(
            &format!("UPDATE classes SET {q} WHERE class_hash='{class_hash}'"))
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
            Class, "SELECT class_hash, class_name, rsl, enable_whitelist FROM classes WHERE class_hash=?", class_hash
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

    // The get_class_announcements() function is used
    // to get all the announcements a teacher has
    // made within provided class_hash.
    async fn get_class_announcements(&self, class_hash: &str) -> Vec<Announcement> {
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

    // The get_unit_lessons() function is used to get all
    // the lesson data that comes with the provided unit hash.
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

    // The get_whitelist_json() function is used to
    // geterate a new json map as a string from the
    // provided whitelist array.
    fn get_whitelist_json(&self, whitelist: Vec<Whitelist>) -> String {
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

    // The get_submission_json() function is used to return
    // a string json map with all the submission data
    // that was retrieved from the database.
    fn get_submission_json(&self, submissions: Vec<Submission>) -> String {
        // Define the json result string
        let mut r: String = String::new();
        // Iterate over the provided submissions array and
        // append each of the lesson's data to a formatted
        // string array of maps
        submissions.iter().for_each(|f| {
            r.push_str(
                &format!("{{
                    \"submitter_hash\": \"{}\", 
                    \"submission_hash\": \"{}\", 
                    \"submission_date\": \"{}\", 
                    \"data\":\"{}\"
                }},",
                f.submitter_hash, f.submission_hash, f.submission_date, f.data
            ))
        });
        // Remove the last comma of the string array
        // before returning the new json map result
        return r[..r.len()-1].to_string();
    }
    
    // The get_unit_lesson_json() function converts the
    // array of lessons into a readable json map that
    // will eventually be returned with the outgoing response body
    fn get_unit_lesson_json(&self, lessons: Vec<Lesson>) -> String {
        // Define the json result string
        let mut r: String = String::new();
        // Iterate over the provided lessons array and
        // append each of the lesson's data to a formatted
        // string array of maps
        lessons.iter().for_each(|f| {
            r.push_str(
                &format!("{{
                    \"title\": \"{}\", 
                    \"description\":\"{}\", 
                    \"video\": \"{}\", 
                    \"work\":\"{}\", 
                    \"work_solutions\":\"{}\"
                }},",
                f.title, f.description, f.video, f.work, f.work_solutions
            ))
        });
        // Remove the last comma of the string array
        // before returning the new json map result
        return r[..r.len()-1].to_string();
    }

    // The get_units_json() function is used to generate 
    // a new json map as a string from the provided units array.
    async fn get_units_json(&self, units: Vec<Unit>) -> String {
        // Define the json result string
        let mut r: String = String::new();
        // Iterate over the provided units array and
        // append each of the units data to a formatted
        // string array of maps
        for u in units {
            // Get the lessons that correspond with the unit
            let l: Vec<Lesson> = self.get_unit_lessons(&u.unit_hash).await;
            // Append the unit json to the result string
            r.push_str(
                &format!("{{
                    \"unit_name\": \"{}\", 
                    \"locked\": {}, 
                    \"lessons\": [{}]
                }},", 
                u.unit_name, u.locked==1, self.get_unit_lesson_json(l)
            ));
        };
        // Remove the last comma of the string array
        // before returning the new json map result
        return r[..r.len()-1].to_string();
    }

    // The get_announcements_json() function is used to
    // generate a new json map as a string from the
    // provided announcements array.
    fn get_announcements_json(&self, announcements: Vec<Announcement>) -> String {
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

    // The get_class_data() function is used to get all data
    // revolving around the provided class_hash. This includes
    // the class's primary data (shown below) and the class's
    // units and lessons.
    pub async fn get_class_data(&self, class_hash: &str) -> String {
        let class:          Option<Class> = self.get_class_basic_data(class_hash).await;
        let units:          Vec<Unit> = self.get_class_units(class_hash).await;
        let whitelist:      Vec<Whitelist> = self.get_class_whitelist(class_hash).await;
        let announcements:  Vec<Announcement> = self.get_class_announcements(class_hash).await;
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