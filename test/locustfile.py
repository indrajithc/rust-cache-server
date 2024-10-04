import redis
from locust import User, task, between

class RedisUser(User):
    wait_time = between(1, 3)  # Simulate wait time between tasks

    def on_start(self):
        # Connect to Redis server
        self.redis_client = redis.StrictRedis(host='localhost', port=6379, db=0)
        # Assign a unique user_id using the user ID of the Locust instance
        self.user_id = self.environment.runner.user_count + 1  # Start at 1

    @task
    def set_cache(self):
        # Set a cache entry
        unique_key = f"myKey-{self.user_id}"
        self.redis_client.set(unique_key, "myValue")

    @task
    def get_cache(self):
        # Get a cache entry
        unique_key = f"myKey-{self.user_id}"
        value = self.redis_client.get(unique_key)
        if value:
            print(f"Retrieved {value.decode('utf-8')} for key {unique_key}")
        else:
            print(f"Key {unique_key} not found.")

# To run: locust -f locustfile.py -u 1000 -r 100
