# document-server

Sample Rust service based on actix as a web server with diesel as ORM that has a documents resource for getting and storing documents via JSON.

## Building

Just use
```
cargo build
```
in the root of the project. Sqlite3 comes as a bundled version so you don't need a database driver preinstalled.

## Running

Before the app can be run diesel needs to create the database. This can be done by running
```
diesel setup
```
to create an empty database file called `document-server.db`. The file name can be controlled by setting the `DATABASE_URL` environment variable to another file.

**To create the schema needed run:**
```
diesel migration run
```

After the database has been set up just run
```
cargo run
```
in the root of the project. This will start a HTTP service on `http://localhost:9090` with a single `documents` resource which accepts `GET` requests to get all documents as a JSON array nad `POST` to post a single document as a JSON object containing only the name.

## Requests

To make requests you can use `curl`. 

**Get all documents:**
```
curl http://localhost:9090/documents
```

**Store a document:**
```
curl -X POST -H "Content-Type: application/json" http://localhost:9090/documents -d "{\"name\":\"sample\"}"
```
*Note that this endpoint currently only returns the rows affected by the insert statement, normally this is always 1. It should return the id of the created document in the future.*