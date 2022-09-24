// Database Struct for globalizing it's
// connection variable
pub struct Database { conn: sqlx::SqlitePool }

// Store The User data as a struct
#[derive(Debug)]
pub struct User {
    pub id:             i64,        // Row Increment ID
    pub hash:       String,     // The user hash (aka: the user id)
    pub name:       String,     // The user name
    pub rsl:        i64,        // Whether the user has "Require Student Login" Enabled
    pub analytics:  i64         // Whether the user has "Analytics" Enabled
}

// Database Implemenetation that contains all the
// functions for manipulating the sqlite db data
impl Database {
    // Initialize a new database connection
    pub async fn init() -> Database {
        // Initialize a connection to the sqlite database
        return Database{ conn: sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .filename("database.sqlite")
                    .create_if_missing(true),
            )
            .await
            .expect("Couldn't connect to database") }
    }

    // The insert_test_user() function is used to
    // insert a fake user for testing the backend
    // database functions
    pub async fn insert_test_user(&self) -> sqlx::sqlite::SqliteQueryResult {
        return sqlx::query!( 
            "INSERT INTO users (user_hash, user_name, rsl, analytics) VALUES (?, ?, ?, ?)",
            "822f3d5b9c91b570a4f1848c5d147b4709d2fb96", "realtristan", 0, 0
        ).execute(&self.conn).await.unwrap();
    }

    // The query_user_by_hash() function is used to query
    // the database for an user with the provided hash
    // Once found, the function will return the users
    // name, rsl(bool), analytics(bool) and id
    pub async fn query_user_by_hash(&self, user_hash: &str) -> User {
        // Query the database
        let r = sqlx::query!(
            "SELECT id, user_name, rsl, analytics FROM users WHERE user_hash=?",
            user_hash
        ).fetch_one(&self.conn).await.unwrap();

        // Return the 'User' object containing all of
        // the requested user's data
        return User {
            id: r.id,
            hash: user_hash.to_string(),
            name: r.user_name,
            rsl: r.rsl,
            analytics: r.analytics
        }
    }
}
