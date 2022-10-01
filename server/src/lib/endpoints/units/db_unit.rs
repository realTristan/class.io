use actix_web::web::Json;
use crate::lib::{self, global};
use super::endp_unit::UnitDataBody;

// The Lesson data struct is used to store
// the class unit's lesson title, description,
// video_url, work and work_solutions.
pub struct Lesson {
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
// The Unit data struct is used to store
// the class unit's unique identifier, 
// unit name, it's locked status and the
// lessons that come along with the unit.
pub struct Unit {
    // The unique unit identifier
    unit_hash: String,
    // The Unit's Name
    unit_name: String,
    // Whether students can access this unit yet
    locked: i64
}

// Database Implementation
impl lib::handlers::Database {
    // The get_class_units() function is used to
    // easily get all the units corresponding with
    // the provided class_hash.
    pub async fn get_class_units(&self, class_hash: &str) -> Vec<Unit> {
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

    // The insert_class_unit() function is used to insert a new
    // unit into the database for the provided class. Students who
    // visit the class through the website, will see this unit appear.
    pub async fn insert_class_unit(&self, class_hash: &str, data: &Json<UnitDataBody>) -> u64 {
        // Generate a new unit hash using the provided class_hash
        let unit_hash: String = global::generate_new_hash(class_hash);

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
    pub async fn delete_class_unit(&self, data: &Json<UnitDataBody>) -> u64 {
        let r = sqlx::query!(
            "DELETE FROM units WHERE unit_hash=?", data.unit_hash
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
    pub async fn update_class_unit(&self, data: &Json<UnitDataBody>) -> u64 {
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

    // The get_units_json() function is used to generate 
    // a new json map as a string from the provided units array.
    pub async fn get_units_json(&self, units: Vec<Unit>) -> String {
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
}