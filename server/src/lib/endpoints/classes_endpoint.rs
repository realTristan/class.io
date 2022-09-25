// Here there is
// - Whitelists
// - Submissions
// - Classes
// - Units
// - Announcements

/*

Example 1:
    fn get_class_data(class_hash: &str) {
        return whitelist[String Array], announcements, rsl, analytics, class_name, 
        "units": [
            "unit_name": {
                "is_locked": bool
                "lessons": [
                    { 
                        "title": "",
                        "description", "",
                        "video_url": "",
                        "etc...": ""
                    }
                ],
            }
        ]
    }

*/

/*

Example 2:
    - USE BEARER TOKEN FOR ACCESSING THIS

    
    fn get_unit_submissions(
        Optional<unit_hash: &str>,
        Optional<user_hash: &str>
    ) {
        return [
            { submission_date: submission }
        ]
    }

*/