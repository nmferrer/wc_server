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


