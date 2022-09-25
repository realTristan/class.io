import requests, time, hashlib

# // Constant Variables
# // USER_HASH: str -> User's ID
USER_HASH: str = "822f3d5b9c91b570a4f1848c5d147b4709d2fb96"
# // SSUPER_SECRET_CODE: str -> Secret Code for Preventing Abuse
SUPER_SECRET_CODE: str = "super_secret_code"

# // Function used for SHA256 encryption
def sha256_encode(v: str):
    return hashlib.sha256(v.encode('utf-8')).hexdigest()

# // Create a formatted auth string
generated_auth:str = f"{USER_HASH}:{int(time.time())}:{SUPER_SECRET_CODE}"
# // Encode that string using SHA256 encryption
auth_token = sha256_encode(generated_auth)
# // Track api latency
start_time = time.time()

# // Send the http request to the api
r = requests.get(f"http://127.0.0.1:8000/user/get/{USER_HASH}/{auth_token}")
print(f" >> Response: {time.time()-start_time} -> {r.text}")
# {
    # "auth_token": "1ed4c5700b434be84953a6052dfd0357aecf99480a0a8d2415528ce19bb9383c", 
    # "user_hash": "822f3d5b9c91b570a4f1848c5d147b4709d2fb96", 
    # "user_name": "realtristan", 
    # "user_rsl": false, 
    # "user_analytics": false
# }
