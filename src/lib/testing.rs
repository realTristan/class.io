use super::handlers;

impl handlers::Database {
    // The insert_test_class() function is used for endpoint
    // debugging as it is required that atleast one class be
    // present in order to properly test.
    pub async fn insert_test_class(&self) 
    {
        println!("Test User Hash: 22f3d5b9c91b570a4f1848c5d147b4709d2fb96");
        println!("Test Class Hash: e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020");

        // Insert into CLASSES column
        sqlx::query!(
            "INSERT INTO classes (owner_id, owner_bearer, class_id, class_name, enable_whitelist) VALUES (?, ?, ?, ?, ?)",
            "owner_id", "822f3d5b9c91b570a4f1848c5d147b4709d2fb96", "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "Advanced Functions", 0
        ).execute(&self.conn).await.unwrap();

        // Insert into ANNOUNCEMENTS column
        sqlx::query!(
            "INSERT INTO announcements (class_id, announcement_id, author_name, title, description, attachment, date) VALUES (?, ?, ?, ?, ?, ?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "announcement_id", "Tristan Simpson", "Test Announcement", "Hey guys!", "no_attachment", 0
        ).execute(&self.conn).await.unwrap();

        // Insert into LESSONS column
        sqlx::query!(
            "INSERT INTO lessons (unit_id, title, description, video, work, work_solutions) VALUES (?, ?, ?, ?, ?, ?)",
            "random_unit_id", "test_lesson_title", "test_lesson_desc", "test_lesson_video", "test_lesson_work", "test_lesson_work_solutions"
        ).execute(&self.conn).await.unwrap();

        // Insert into UNITS column
        sqlx::query!(
            "INSERT INTO units (class_id, unit_id, unit_name, locked) VALUES (?, ?, ?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020",
            "random_unit_id",
            "Polynomials",
            0
        ).execute(&self.conn).await.unwrap();

        // Insert into UNITS column
        sqlx::query!(
            "INSERT INTO units (class_id, unit_id, unit_name, locked) VALUES (?, ?, ?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020",
            "random_unit_id",
            "Functions",
            0
        ).execute(&self.conn).await.unwrap();

        // Insert into UNITS column
        sqlx::query!(
            "INSERT INTO units (class_id, unit_id, unit_name, locked) VALUES (?, ?, ?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020",
            "random_unit_id",
            "Calculus",
            0
        ).execute(&self.conn).await.unwrap();

        // Insert into SUBMISSIONS column
        sqlx::query!(
            "INSERT INTO submissions (class_id, submission_id, submitter_bearer, submission_date, data) VALUES (?, ?, ?, ?, ?)",
            "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", "submission_id", "822f3d5b9c91b570a4f1848c5d147b4709d2fb96", 0, ""
        ).execute(&self.conn).await.unwrap();
    }
}
