use crate::lib;

// Database Implementation
impl lib::handlers::Database {

    // The insert_class_unit() function is used to insert a new
    // unit into the database for the provided class. Students who
    // visit the class through the website, will see this unit appear.
    pub async fn insert_class_unit(&self, bearer: &str, unit_id: &str, class_id: &str, unit_name: &str) -> bool 
    {
        // If the unit already exists, return false
        if self.unit_exists(unit_id).await {
            return false;
        }

        // Insert the data into the database
        let query = sqlx::query!(
            "INSERT INTO units (owner_bearer, class_id, unit_id, unit_name, locked) VALUES (?, ?, ?, ?, ?)", 
            bearer, class_id, unit_id, unit_name, 0
        ).execute(&self.conn).await;

        // Return the result of the query
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false,
        };
    }

    // The unit_exists() function is used to check whether
    // the provided unit hash already exists. This function
    // is called in the insert_class_unit() function.
    async fn unit_exists(&self, unit_id: &str) -> bool 
    {
        // Query the database
        let query = sqlx::query!("SELECT * FROM units WHERE unit_id=?", unit_id)
            .fetch_one(&self.conn)
            .await;

        // Return whether valid query data has been obtained
        return !query.is_err();
    }

    // The delete_class_unit() function is used to delete a unit
    // from the units column wherever the provided unit_id
    // is present. A maximum of 12 units is allowed per class.
    pub async fn delete_class_unit(
        &self, bearer: &str, class_id: &str, unit_id: &str
    ) -> bool {

        // Query the database
        let query = sqlx::query!(
            "DELETE FROM units WHERE unit_id=? AND owner_bearer=? AND class_id=?",
            unit_id, bearer, class_id
        ).execute(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false,
        };
    }

    // The generate_unit_update_query() function is used to generate
    // a string that will be used for updating the unit data within
    // the database. The function takes the request body as a parameter
    // and returns a string that will be used in the update query.
    async fn generate_unit_update_query(&self, data: serde_json::Value) -> String {
        // Create a new string
        let mut query: String = String::new();

        // Get the unit name from the request body
        match data.get("user_name") {
            Some(name) => query.push_str(&format!("unit_name='{}',", name)),
            None => ()
        };
        // Get the locked bool from the request body
        match data.get("locked") {
            Some(locked) => query.push_str(&format!("locked={},", locked.to_string())),
            None => ()
        };

        // Remove the trailing comma
        return query[..query.len() - 1].to_string();
    }

    // The update_class_unit() function is used to update a class's
    // unit data replacing the current data, with that of the provided
    // Json<UnitDataBody> values. In order to prevent null values
    // being updated, the function first determines which values are null.
    pub async fn update_class_unit(
        &self, bearer: &str, class_id: &str, unit_id: &str, data: serde_json::Value
    ) -> bool {
        // Generate the query update data
        let query_data: String = self.generate_unit_update_query(data).await;

        // Query the database, updating all the values
        // in the above query_data: String that have the same
        // unit_id as the one provided
        let query = sqlx::query(&format!(
            "UPDATE units SET {} WHERE unit_id='{}' AND owner_bearer='{}' AND class_id='{}'",
            query_data, unit_id, bearer, class_id
        )).execute(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false,
        };
    }
}
