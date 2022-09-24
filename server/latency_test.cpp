#include <curl/curl.h>
#include <iostream>
#include <chrono>

// The get_time_since_epoch() function is used to return
// the milliseconds since epoch. This number is used to
// determine how fast the http request was
using namespace std::chrono;
milliseconds get_time_since_epoch() {
    return duration_cast< milliseconds >(
        system_clock::now().time_since_epoch()
    );
}

// Compile: gcc -o latency_test latency_test.c -lcurl
int main(void) {
    // Initialize the request
    CURL *curl;
    curl_global_init(CURL_GLOBAL_ALL);
    curl = curl_easy_init();

    // Start time
    milliseconds start_time = get_time_since_epoch();
    if (curl) {
        // Send http request
        curl_easy_setopt(curl, CURLOPT_URL, 
            "http://127.0.0.1:8000/822f3d5b9c91b570a4f1848c5d147b4709d2fb96/no_auth");
        curl_easy_perform(curl); // Perform http request
        curl_easy_cleanup(curl); // Cleanup local
    }
    // Print the total time it took to send the http request
    milliseconds end_time = get_time_since_epoch();
    std::cout << (end_time-start_time).count() << std::endl;

    // Cleanup Global
    curl_global_cleanup();
    return 0;
}