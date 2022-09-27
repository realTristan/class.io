from tracemalloc import start
import requests, time

start_time: float = time.time()
r = requests.get("http://127.0.0.1:8080/class/e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020")
print(time.time()-start_time)