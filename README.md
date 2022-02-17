#Rust API Demo

###Installation
- make sure you have rust installed.
- make sure you have sqlx installed.
- make a copy of .env.example as .env
- replace what is needed.

###Development
- run `sqlx database create` to create your database file.
- run `sqlx migrate run` to apply migrations.
- run `cargo run` to start your server.

###APIs
- GET: `curl -i -X GET -H "Content-Type: application/json" http://YOUR_HOST:YOUR_PORT/users` lists all users
- POST: `curl -i -X POST -H "Content-Type: application/json" -d '{"email":"anyEmail@host.com", "phone": "0599033975"}' http://YOUR_HOST:YOUR_PORT/users` creates a new user
- GET: `curl -i -X GET -H "Content-Type: application/json" http://YOUR_HOST:YOUR_PORT/users/{USER_ID}` retrieve single user by id
- PUT: `curl -i -X PUT -H "Content-Type: application/json" -d '{"email":"anyEmail@host.com", "phone": "0599033975"}' http://YOUR_HOST:YOUR_PORT/users/{USER_ID}` updates user by id.
