use crate::lib::{
    self, structs::UnitDataBody
};
use actix_web::web::Json;

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

    // The update_class_unit() function is used to update a class's
    // unit data replacing the current data, with that of the provided
    // Json<UnitDataBody> values. In order to prevent null values
    // being updated, the function first determines which values are null.
    pub async fn update_class_unit(
        &self, bearer: &str, class_id: &str, unit_id: &str, data: &Json<UnitDataBody>
    ) -> bool {

        // Create a new string
        let mut query_data: String = String::new();
        
        // If provided unit_name
        if data.unit_name.len() > 0 {
            query_data.push_str(&format!("unit_name='{}',", data.unit_name));
        }

        // If provided locked bool
        if data.locked != 2 {
            query_data.push_str(&format!("locked={},", data.locked));
        }

        // Remove the trailing comma
        let query_data: String = query_data[..query_data.len() - 1].to_string();

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
