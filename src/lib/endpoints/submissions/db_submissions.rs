use crate::lib::{self, global};

// The Submission data struct is used to store
// the work: submitter_bearer, submissions_id,
// submission_date and work data
struct Submission {
    // The user who submitted the work's unique identifier
    submitter_bearer: String,
    // The unique identifier of the submission
    submission_id: String,
    // The date the work was submitted
    submission_date: i64,
    // The submission data. (ex: the file, the answers, etc.)
    data: String,
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
        submissions.iter().for_each(|s| {
            r.push_str(&format!(
                "{{
                    \"submitter_bearer\": \"{}\", 
                    \"submission_id\": \"{}\", 
                    \"submission_date\": \"{}\", 
                    \"data\":\"{}\"
                }},",
                s.submitter_bearer, s.submission_id, s.submission_date, s.data
            ))
        });
        // Remove the last comma of the string array
        // before returning the new json map result
        return r[..r.len() - 1].to_string();
    }

    // The insert_class_submission() function is used to
    // insert a new work submission into the database
    // using the provided class hash. The function generates
    // a unique submission hash before inserting the data, which
    // is used within the delete_class_submission() function
    pub async fn insert_class_submission(
        &self,
        class_id: &str,
        submission_id: &str,
        submitter_bearer: &str,
        data: &str,
    ) -> u64 {
        // If the submission already exists, return
        if self.class_submission_exists(submission_id).await {
            return 0;
        }

        // Get the current date to put into the database
        let date: i64 = global::get_time() as i64;

        // Insert the data into the database
        let r = sqlx::query!(
            "INSERT INTO submissions (class_id, submission_id, submitter_bearer, submission_date, data) VALUES (?, ?, ?, ?, ?)", 
            class_id, submission_id, submitter_bearer, date, data
        ).execute(&self.conn).await;

        // If an error has occurred, return 0 rows affected
        if r.is_err() {
            return 0;
        }

        // Else, return the actual amount of rows that
        // have been affected by the insertion
        return r.unwrap().rows_affected();
    }

    // The class_submission_exists() function is used to check whether
    // the provided submission hash already exists. This function
    // is called in the insert_class_submission() function.
    async fn class_submission_exists(&self, submission_id: &str) -> bool {
        // Query the database
        let r = sqlx::query!(
            "SELECT * FROM submissions WHERE submission_id=?",
            submission_id
        ).fetch_one(&self.conn).await;

        // Return whether valid query data has been obtained
        return !r.is_err();
    }

    // The delete_class_submission() function is used to
    // delete a submission from the database. This function
    // is called when a student wants to unsubmit a portion
    // of their work.
    pub async fn delete_class_submission(
        &self,
        submitter_bearer: &str,
        submission_id: &str,
    ) -> u64 {
        // Query the database, deleting all data revolving around
        // the provided submission hash
        let r = sqlx::query!(
            "DELETE FROM submissions WHERE submission_id=? AND submitter_bearer=?",
            submission_id,
            submitter_bearer
        ).execute(&self.conn).await;

        // If an error has occurred, return 0 rows affected
        if r.is_err() {
            return 0;
        }

        // Else, return the actual amount of rows that
        // have been affected by the insertion
        return r.unwrap().rows_affected();
    }

    // The get_class_submissions() function is used to
    // return all the submissions for the provided class.
    // This function is used in the dashboard of the website
    // where the teachers can mark the students submitted work.
    pub async fn get_class_submissions(&self, class_id: &str) -> String {
        // Query the database, selecting the submitter_bearer, submission_date
        // and the submission data from the submissions column
        let r = sqlx::query_as!(
            Submission, "SELECT submitter_bearer, submission_id, submission_date, data FROM submissions WHERE class_id=?", 
            class_id
        ).fetch_all(&self.conn).await;

        // Return empty if an error has occurred
        if r.is_err() {
            return "{}".to_string();
        }

        // Else if no error has occurred, return
        // the unwrapped array of all the units
        return format!("[{}]", self.get_submission_json(r.unwrap()));
    }

    // The get_user_submissions() function is used to get all the
    // submissions from the provided bearer within the provided
    // class. This function is used for the students to see
    // what work they have previously submitted. This function is the
    // basis towards students being able to insert and delete submissions.
    pub async fn get_user_submissions(&self, class_id: &str, bearer: &str) -> String {
        // Query the database selecting the submitter_bearer, submission_id, submission_date
        // and the submission data from the submissions column.
        let r = sqlx::query_as!(
            Submission,
            "SELECT submitter_bearer, submission_id, submission_date, data 
                            FROM submissions WHERE class_id=? AND submitter_bearer=?",
            class_id,
            bearer
        ).fetch_all(&self.conn).await;

        // Return empty if an error has occurred
        if r.is_err() {
            return "{}".to_string();
        }
        
        // Else if no error has occurred, return
        // the unwrapped array of all the units
        return format!("[{}]", self.get_submission_json(r.unwrap()));
    }
}
