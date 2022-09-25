
// Example:
// split("user_name:title:description:attached_image", ":")
struct Announcement {
    author_name: String,
    title: String,
    description: String,
    attachements: Vec<String>   // Base64 encode images, etc.

}

// Store the class data
struct Class {
    owner_hash:             String,
    class_hash:             String,
    class_name:             String,
    whitelist:              Vec<String>,
    announcements:          Vec<Announcement>,
    require_student_login:  bool,
    analytics:              bool,
    units:                  Vec<String>         // Array of unit_hashes
}

// Store the unit data
struct Unit {
    unit_hash:      String,
    unit_name:      String,
    locked:         bool,
    lessons:        Vec<Lesson>
}

// Store the lesson data
struct Lesson {
    unit_hash:      String,
    title:          String,
    description:    String,
    video_url:      String, // and so on..
}

/*

use bearer token when updating the database
and
also use the auth token so the user can't abuse the api

BEARER TOKEN IS SHA256 ENCODE [ (user_hash):(super_secret_bearer_code):(provided auth token):(registration_date) ]

*/