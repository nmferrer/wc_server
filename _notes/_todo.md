01/25/2026
-Configure Database (SQLx, Tokio). Populate from data sourced from NOAA.
-Write API for clients to interact with server. Adhere to REST protocol for CRUD operations.
-Refine CLI client. Logic checks, data integrity, test.

01/28/2026
Database Setup
-SQLx, Tokio
-Confirm db initializes properly
-Confrim db runs on server {ip:port}
-Confirm CRUD operations on local instance
REST Setup
-Rust TCP/IP integration
-Confirm read/write operations to stream
-Confirm listening and closing of connections

01/30/2026
Database Server setup
    * Figure out how to install on local/remote machine
    * Connect to db server via ???

02/02/2026
Continued: Database Setup.
    * Have database_server running as a background process?
    * Weather_Server connects to database through connection pool?

02/03/2026
SQLite is installed. Configure.
How do I spin up local instance? Where is database stored? Is it persistent?
    * Once I figure out where database is stored, I can interface with rust via SQLx.
    * Answer: When specified, sqlite3 creates a SQLite database file. Perform operations through SQLx.

02/05/2026:
Linking crates: Server, Client, Database, Polling Data
    * See *The Rust Programming Language* (Chapter 7) *Rust by Example* (Chapter 12 - Cargo)

Client: Consider CLAP (Command Line Argument Parser)
Testing:
    * Client-Server TCP/IP Connection
    * Server can connect to database
    * Server can r/w to database & SQLx queries function as intended
