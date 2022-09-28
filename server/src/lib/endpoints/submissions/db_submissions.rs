use actix_web::web::Json;
use crate::lib;
use super::endp_submissions::SubmissionDataBody;

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

// Database Implementation
impl lib::handlers::Database {
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
}