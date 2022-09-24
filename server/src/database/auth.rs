use std::collections::HashMap;
use std::sync::Mutex;

// The SUPER_SECRET_CODE is what's used to prevent
// users trying to abuse the api from being able
// to generate their own auth tokens
static SUPER_SECRET_CODE: &str = "super_secret_code";

// The TOKEN_STORAGE is used to store previously used
// tokens so that abusers can't access the api using
// a previous token.
lazy_static::lazy_static! {
    #[derive(Debug)]
    static ref TOKEN_STORAGE: Mutex<HashMap<String, Vec<String>>> = {
        Mutex::new(HashMap::new())
    };    
}

// The storage_handler() function is used to check whether
// the provided auth token has already been used
// within the past 8 seconds. This is function is
// necessary to prevent abusers from using the same
// token more than once.
fn storage_handler(user_hash: &str, auth_token: &str, time: &u64) -> bool {
    let mut token_storage = TOKEN_STORAGE.lock().unwrap();

    // Convert the token storage into a mutable variable.
    // This is required so that we can append the auth_token
    // to the users token storage, or so that we can clear
    // the token storage if full.
    let mut_storage: Option<&mut Vec<String>> = token_storage.get_mut(user_hash);

    // If the user doesn't already exist within the
    // token storage.. Insert a new key:value that
    // contains the users hash and the array containing the
    // current time (which will be used to determine the last wipe)
    // and the provided auth token.
    if mut_storage.is_none() {
        // Insert the user into the token storage
        // along with the current time and auth token
        token_storage.insert(
            user_hash.to_string(), 
            [time.to_string(), auth_token.to_string()].to_vec()
        );
        // Return true as the token did not
        // previously exist in the token storage
        return true;
    }
    // Unwrap the previous mutable storage
    let mut_storage: &mut Vec<String> = mut_storage.unwrap();

    // Get the last storage wipe time
    let last_wipe_time: u64 = mut_storage.first().unwrap().parse().expect("");

    // If the last wipe happened over 8 seconds ago,
    // wipe the users token storage to prevent an
    // overflow. If the user has too many tokens and
    // the cache isn't eventually cleared.. you already
    // know what'll happen lmao.
    if time > &(last_wipe_time+8) {
        mut_storage.clear();
        mut_storage.push(time.to_string());
    }
    
    // After the users current token storage has or hasn't been
    // cleared, check whether the auth_token is already existant
    // in the token storage. If it is, return false, thus the
    // user is using an unauthorized token. Else, append the
    // token to the user's token storage and return true.
    if !mut_storage.contains(&auth_token.to_string()) {
        mut_storage.push(auth_token.to_string());
        return true;
    }
    // Debugging
    println!("\n\nUser Token Storage: {:?}\n\n", token_storage);
    return false;
}

// The verify() function is used to check whether the
// provided auth token is valid. It does this by
// checking whether the token has been created within
// the past 8 seconds. If so, return true, else, return false.
pub fn verify(user_hash: &str, auth_token: &str) -> bool {
    // Get the system time since epoch. This value
    // is used to check how long ago the auth token was
    // generated. Doing this prevents users from consecutively
    // using a single auth token if trying to abuse the api
    let time: std::time::Duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap();
    let time = time.as_secs();

    // Execute the storage handler
    // If the function returns false, then the provided
    // auth token has already been used within the past 8 seconds.
    if !storage_handler(user_hash, auth_token, &time) { return false };

    // Check whether the auth token was generated
    // within the past 10 seconds
    for i in 0..8 {
        let generated_auth: String = format!("{}:{}:{}", user_hash, time-i, SUPER_SECRET_CODE);
        // If the provided auth token is equal to the
        // generated auth token, return true
        if auth_token == sha256::digest(generated_auth) {
            return true;
        }
    }
    return false;
}