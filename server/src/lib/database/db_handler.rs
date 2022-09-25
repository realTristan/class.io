// Database Struct for globalizing it's
// connection variable
pub struct Database { pub conn: sqlx::SqlitePool }

// Database Implemenetation that contains all the
// functions for manipulating the sqlite db data
#[allow(dead_code)]
impl Database {
    // Initialize a new database connection
    pub async fn init() -> Self {
        // Initialize a connection to the sqlite database
        return Self{ conn: sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .filename("database.sqlite")
                    .create_if_missing(true),
            )
            .await
            .expect("Couldn't connect to database") }
    }
}
