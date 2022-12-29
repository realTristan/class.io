use crate::lib::{
    self, structs::{Unit, UnitDataBody}
};
use actix_web::web::Json;

// Database Implementation
impl lib::handlers::Database {
    // The get_class_units() function is used to
    // easily get all the units corresponding with
    // the provided class_id.
    pub async fn get_class_units(&self, class_id: &str) -> Vec<Unit> 
    {
        // Fetch all the units that are in the class.
        // By the end of the function, all the unit data
        // will be neatly categorized.
        let query = sqlx::query_as!(
            Unit,
            "SELECT unit_id, unit_name, locked FROM units WHERE class_id=?",
            class_id
        ).fetch_all(&self.conn).await;

        // Return the result of the query
        return match query {
            Ok(r) => r,
            Err(_) => Vec::new(),
        }
    }

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
    pub async fn delete_class_unit(&self, bearer: &str, unit_id: &str) -> bool 
    {
        let query = sqlx::query!(
            "DELETE FROM units WHERE unit_id=? AND owner_bearer=?",
            unit_id, bearer
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
    pub async fn update_class_unit(&self, bearer: &str, data: &Json<UnitDataBody>) -> bool 
    {
        let mut res: String = String::new();
        // If the provided data's enable_whitelist integer bool
        // isn't invalid (equal to 2) then append the
        // updated value to the result string
        if data.unit_name.len() > 0 {
            res.push_str(&format!("unit_name='{}',", data.unit_name));
        }

        // If the provided data's locked integer bool
        // isn't invalid (equal to 2) then append the
        // updated value to the result string
        if data.locked != 2 {
            // 2 == Invalid
            res.push_str(&format!("locked={},", data.locked));
        }

        // Remove the trailing comma
        let res: String = res[..res.len() - 1].to_string();

        // Query the database, updating all the values
        // in the above res: String that have the same
        // unit_id as the one provided
        let query = sqlx::query(&format!(
            "UPDATE units SET {} WHERE unit_id='{}' AND owner_bearer='{}'",
            res, data.unit_id, bearer
        )).execute(&self.conn).await;

        // Return query result
        return match query {
            Ok(r) => r.rows_affected() > 0,
            Err(_) => false,
        };
    }
}
