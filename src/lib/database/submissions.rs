use crate::lib::{self, global, structs::Submission};

// Database Implementation
impl lib::handlers::Database {
    // The class_submission_exists() function is used to check whether
    // the provided submission hash already exists. This function
    // is called in the insert_class_submission() function.
    async fn class_submission_exists(&self, submission_id: &str) -> bool 
    {
        // Query the database
        let query = sqlx::query!(
            "SELECT * FROM submissions WHERE submission_id=?",
            submission_id
        ).fetch_one(&self.conn).await;

        // Return whether valid query data has been obtained
        return !query.is_err();
    }

    // The insert_class_submission() function is used to
    // insert a new work submission into the database
    // using the provided class hash. The function generates
    // a unique submission hash before inserting the data, which
    // is used within the delete_class_submission() function
    pub async fn insert_class_submission(&self, class_id: &str,  submission_id: &str, submitter_bearer: &str, data: &str) -> bool 
    {
        // If the submission already exists, return
        if self.class_submission_exists(submission_id).await {
            return false;
        }

        // Get the current date to put into the database
        let date: i64 = global::get_time().as_secs() as i64;

        // Insert the data into the database
        let query = sqlx::query!(
            "INSERT INTO submissions (class_id, submission_id, submitter_bearer, submission_date, data) VALUES (?, ?, ?, ?, ?)", 
            class_id, submission_id, submitter_bearer, date, data
        ).execute(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false
        };
    }

    // The delete_class_submission() function is used to
    // delete a submission from the database. This function
    // is called when a student wants to unsubmit a portion
    // of their work.
    pub async fn delete_class_submission(
        &self, submitter_bearer: &str, class_id: &str, submission_id: &str
    ) -> bool {
        // Query the database, deleting all data revolving around
        // the provided submission hash
        let query = sqlx::query!(
            "DELETE FROM submissions WHERE submission_id=? AND submitter_bearer=? AND class_id=?",
            submission_id, submitter_bearer, class_id
        ).execute(&self.conn).await;

        // Return query result
        return match query {
            // If an error has occurred, return 0 rows affected
            Err(_) => false,
            // Else, return the actual amount of rows that
            // have been affected by the deletion
            Ok(r) => r.rows_affected() > 0,
        };
    }

    
    // The get_submission_json() function is used to return
    // a string json map with all the submission data
    // that was retrieved from the database.
    fn get_submission_json(&self, submissions: Vec<Submission>) -> Vec<serde_json::Value> 
    {
        return submissions.iter().map(|s| {
            serde_json::json!({
                "submitter_bearer": s.submitter_bearer,
                "submission_id": s.submission_id,
                "submission_date": s.submission_date,
                "data": s.data
            })
        }).collect();
    }

    // The get_class_submissions() function is used to
    // return all the submissions for the provided class.
    // This function is used in the dashboard of the website
    // where the teachers can mark the students submitted work.
    pub async fn get_class_submissions(&self, class_id: &str) -> Option<Vec<serde_json::Value>> 
    {
        // Query the database, selecting the submitter_bearer, submission_date
        // and the submission data from the submissions column
        let query = sqlx::query_as!(Submission, 
            "SELECT submitter_bearer, submission_id, submission_date, data FROM submissions WHERE class_id=?", 
            class_id
        ).fetch_all(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => Some(self.get_submission_json(r)),
            Err(_) => None
        };
    }

    // The get_user_submissions() function is used to get all the
    // submissions from the provided bearer within the provided
    // class. This function is used for the students to see
    // what work they have previously submitted. This function is the
    // basis towards students being able to insert and delete submissions.
    pub async fn get_user_submissions(&self, class_id: &str, bearer: &str) -> Option<Vec<serde_json::Value>>
    {
        // Query the database selecting the submitter_bearer, submission_id, submission_date
        // and the submission data from the submissions column.
        let query = sqlx::query_as!(Submission,
            "SELECT submitter_bearer, submission_id, submission_date, data FROM submissions WHERE class_id=? AND submitter_bearer=?",
            class_id, bearer
        ).fetch_all(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => Some(self.get_submission_json(r)),
            Err(_) => None
        };
    }
}
