import requests, time, hashlib

# // Constant Variables
# // USER_HASH: str -> User's ID
USER_HASH: str = "822f3d5b9c91b570a4f1848c5d147b4709d2fb96"
# // SUPER_SECRET_CODE: str -> Secret Code for Preventing Abuse
SUPER_SECRET_CODE: str = "super_secret_code"

# // Function used for SHA256 encryption
def sha256_encode(v: str) -> str:
    return hashlib.sha256(v.encode('utf-8')).hexdigest()

# // Test the /user/get endpoint
def test_get():
    # // Create a formatted auth string
    generated_auth:str = f"{USER_HASH}:{int(time.time())}:{SUPER_SECRET_CODE}"
    # // Encode that string using SHA256 encryption
    auth_token = sha256_encode(generated_auth)
    # // Track api latency
    start_time = time.time()

    # // Send the http request to the api
    r = requests.get(f"http://127.0.0.1:8000/user/{USER_HASH}", headers={
        "Access Token": auth_token
    })
    print(f" >> Response: {time.time()-start_time} -> {r.text}")
    # {
        # "auth_token": "1ed4c5700b434be84953a6052dfd0357aecf99480a0a8d2415528ce19bb9383c", 
        # "user_hash": "822f3d5b9c91b570a4f1848c5d147b4709d2fb96", 
        # "user_name": "realtristan", 
        # "user_rsl": false, 
        # "user_analytics": false
    # }


# // Test the /user/update endpoint
def test_update():
    # // Firebase user special token
    firebase_token:str = ""
    
    # // Create a formatted auth string
    generated_auth:str = f"{USER_HASH}:{int(time.time())}:{SUPER_SECRET_CODE}"
    # // Encode that string using SHA256 encryption
    auth_token = sha256_encode(generated_auth)
    # // Create a new bearer token
    bearer: str = sha256_encode(f"{USER_HASH}:{auth_token}:{firebase_token}")
    # // Track api latency
    start_time = time.time()

    # // Send the http request to the api
    r = requests.post(
        f"http://127.0.0.1:8000/user/{USER_HASH}", 
        json={"user_name": "realTristan"},
        headers={"Authorization": f"Bearer {bearer}"}
    )
    print(f" >> Response: {time.time()-start_time} -> {r.text}")
    

# // Run the test functions
if __name__ == "__main__":
    test_update()
    time.sleep(1)
    test_get()