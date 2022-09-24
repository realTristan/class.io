import requests, time
# // Main function
def main():
    # // Establish a start time
    t:float = time.time()
    # // Send request to the api
    requests.get("http://127.0.0.1:8000/822f3d5b9c91b570a4f1848c5d147b4709d2fb96/no_auth")
    # // Print the time since start time
    print(f"{str(time.time()-t)[:7]}s")
# // Run main function
if __name__ == "__main__": main()