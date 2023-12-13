Create a `.env` file and write the following:
```
# .env
USER_NAME=your_user_name
```

Then, start the docker container:
```
docker-compose up -d
```

If you want to run test code, create a `.env` file in the root directory of the Rust project and write as follows:
```
# .env
TESTNET_API_KEY=your_api_key
TESTNET_API_SECRET=your_api_secret
```

After that, execute the following command:
It is recommended that the thread specify 1. because the request frequency will be too high.
```
cargo test -- --test-threads=1
```

# Reference
- [Introduction | Bybit API Documentation](https://bybit-exchange.github.io/docs/v5/intro)