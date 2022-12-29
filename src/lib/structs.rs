
// The User data struct is used to store
// all of the users data from the database
// into readable values
pub struct User {
    // Row Increment ID
    pub id: i64,
    // The unique user identifier
    pub user_id: String,
    // The user hash (aka: the user id)
    pub bearer: String,
    // The users name
    pub user_name: String,
    // The users email
    pub email: String,
    // The Users registration date (used for bearer token)
    pub registration_date: i64,
}

// The Class data struct is used to store
// the classes owner_bearer, unique class identifier,
// class name, class whitelist array, class announcements,
// and the class units.
pub struct Class {
    // The Class Name
    pub class_name: String,
    // Unique class owner identifier
    pub owner_id: String,
    // Whether to the use the class whitelist
    pub enable_whitelist: i64,
}

// The Announcement data struct is used to
// store the announcement author's unique identifier,
// the authors name, the announcement title and description,
// along with any attachments the author has posted with it.
pub struct Announcement {
    // The announcement's author name
    pub author_name: String,
    // The announcement title
    pub title: String,
    // The announcements content
    pub description: String,
    // Any images/videos attached with the announcement
    pub attachment: String, // Base64 encode images, etc.
}

// The Lesson data struct is used to store
// the class unit's lesson title, description,
// video_url, work and work_solutions.
pub struct Lesson {
    // The Lesson Title
    pub title: String,
    // The Lesson Description
    pub description: String,
    // The Lesson's Youtube Video URL
    pub video: String,
    // The Lesson Homework that can be
    // submitted and marked
    pub work: String,
    // The Lesson Homework Solutions
    pub work_solutions: String,
}

// The Unit data struct is used to store
// the class unit's unique identifier,
// unit name, it's locked status and the
// lessons that come along with the unit.
pub struct Unit {
    // The unique unit identifier
    pub unit_id: String,
    // The Unit's Name
    pub unit_name: String,
    // Whether students can access this unit yet
    pub locked: i64,
}

// The Whitelist data struct is used for querying
// the whitelisted users for a specific class.
pub struct Whitelist {
    pub whitelisted_user: String,
}

// The Submission data struct is used to store
// the work: submitter_bearer, submissions_id,
// submission_date and work data
pub struct Submission {
    // The user who submitted the work's unique identifier
    pub submitter_bearer: String,
    // The unique identifier of the submission
    pub submission_id: String,
    // The date the work was submitted
    pub submission_date: i64,
    // The submission data. (ex: the file, the answers, etc.)
    pub data: String,
}

// The UserDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct UserDataBody {
    pub user_name: String,
    pub email: String,
}

// The WhitelistDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct WhitelistDataBody {
    pub user: String,
}

// The SubmissionDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct AnnouncementDataBody {
    // The announcements unique identifier
    pub announcement_id: String,
    // The announcement attachment (image, file, etc.)
    pub attachment: String,
    // The announcement content/description
    pub description: String,
    // The title of the announcement
    pub title: String,
    // The name of the user who posted the announcement
    pub author_name: String,
}

// The ClassDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct ClassDataBody {
    // Update the class name
    pub class_name: String,
    // Update whether to use the class whitelist
    pub enable_whitelist: i64,
}

// The UnitDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct UnitDataBody {
    pub unit_id: String,
    pub unit_name: String,
    pub locked: i64,
}

// The SubmissionDataBody struct is used to read the
// incoming requests http request body. This is
// the easiest way for reading what modifications
// to make within the database
#[derive(serde::Deserialize)]
pub struct SubmissionDataBody {
    pub submission_id: String,
    pub data: String,
}