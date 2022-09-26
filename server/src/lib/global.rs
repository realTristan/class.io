use actix_web::HttpRequest;

// The get_header() function is used to bypass
// any invalid header errors. Without this function,
// if an abuser tried to send a request with an
// invalid error, an internal server error would occur.
pub fn get_header<'a>(req: &'a HttpRequest, header: &str) -> &'a str {
    // Get the header option to check whether the
    // head is valid/present
    let opt_head = req.headers().get(header);
    // If the header is invalid/not-present, return
    // an empty string
    if opt_head.is_none() { return "" }
    // Unwrap the option header and check if
    // it has a valid length. 
    let head_val = opt_head.unwrap();
    // If it doesn't, return an empty string
    if head_val.is_empty() { return "" }
    // Finally return the header as an 
    // unwrapped string
    return head_val.to_str().unwrap()
}

// The get_time() function is used to quickly
// and cleanly get the time in seconds since
// the unix epoch.
pub fn get_time() -> u64 {
    let time: std::time::Duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap();
    return time.as_secs();
}