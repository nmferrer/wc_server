Some thoughts as I make sense of what I need to do + toolset

SQLx: **Database Support**
"SQLx is an async, pure Rust SQL crate featuring compile-time checked queries without a DSL."

Tokio: **Async Runtime**

"A runtime for writing reliable network applications without compromising speed."
"Tokio is an event-driven, non-blocking I/O platform for writing asynchronous applications with the Rust programming language."
    *Asynchronous Tasks (synchronization primitives/channels and timeouts/sleeps/intervals)
    Asynchronous I/O (TCP/UDP, file system operations, process+signal management)
    Runtime for asynchronous code (task scheduler, I/O driver backed by OS event queue)
Dotenv: **Storing of Configuration** 
"This library is meant to be used on development or testing environments in which setting environment variables is not practical. It loads environment variables from an .env file, if available, and mashes those with the actual environment variables provided by the operating system."

Tasks:
**Server**
1. Server Initialized.
2. Database Initialized.
3. Server connects to database.
4. Server listens for clients.
n. Server closes.
**Client (CLI)**
1. User launches client application.
2. Client attempts connection with server.
3. Client is accepted or rejected.
4. Client sends requests.
5. Client receives response.
6. User quits. Client closes connection.

**Extension / Further Learning**
* Web Client (React-Next.js)
* Android Client (Kotlin)

02/02/2026 Some thoughts.
Database: SQLite: Lightweight database engine, operates like file i/o
* Not meant to imitate enterprise-level client-server databases (e.g. MySQL and Postgres) but should be simple and feature rich for personal projects.
* Not every application demands the full feature set of RDBMS.
    -Client/Server, High-volume, Large datasets, High concurrency.
* "Database" for this application only really holds small, fixed number of records.

The main justification for SQLite is that it enables me to become familiar with RDBMS patterns without having to set up entire RDBMS infrastructure
i.e. I can learn proper queries, table structuring and other best practices quicker.
Any further domain knowledge I can learn at work.

Task for 02/02 - 02/04:
Ensure SQLite is setup and accessible by server application.
Client app connects to Server app, but all db transactions are to be done **only** by server.

Workflow:
Tables:
    1. Queries {query_id UNIQUE int, user_id int, {input_data} JSON, {output_data} JSON}
    2. Logs {query_id UNIQUE int, user_id int, {metadata} JSON}
* After unpacking validated client request, process input.
    * If possible, use cached data, i.e. request matches already logged data.
    * Else, server makes request to NOAA API (?), logs to local db, and returns output.

{input_data}    : contents of client request    : method, location, forecast_duration
{output_data}   : contents of server response   : ???
{metadata}      : information about query       : time, date

TODO: read up on api.weather.gov, how to access routes
TODO: best practice for sqlite table structure

**What is to be done?** 02/03
1. SQLite Table Setup: Store queries and log metadata.
    * OK: Creating tables through sqlite3 cli. 
    * Ensure that server is connected to SQLite database on startup. *see sqlx::sqlite::{SqliteConnectOptions}*
    * Use dotenv crate to read environment variables: URI of database.
2. Client-Server Setup: Ensure that server runs and can connect to clients.
3. REST API Setup: Ensure client-server communications adhere to REST protocol.
    * Set up routes for appropriate methods (GET/PUT/POST/DELETE/...)
    * Verify requests and handle successful/unsuccessful transactions and error handling.
4. External Data Setup: Ensure server can populate database tables with data requested from external sources.

**Notes on SQLx example code**
* Package hierarchy: Separation on modules by function.
* TODO: Study how to properly handle separation of concerns / modules.
    * Rust Book: Chapter 7: Packages, Crates, and Modules.
* Review how sqlx functions are used in different database engines.

