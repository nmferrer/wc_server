//Handling of database operations to be executed by server.
//The primary role of the server is to listen for connections and requests and issue appropriate responses.
//The server must be the only user directly connected to SQLite database.
//CRUD (Create, Read, Update, Delete) operations are defined here.
//Database will hold entries of requested information and logging.

//Ensure that server is connected via SQLite Pool
//Pass validated input data as fields of query. Server will have completed validation before passing data.
//On successful query execution, pass data back to server. On failure, handle error and notify.

use sqlx::{
    query,
    sqlite::{SqliteConnectOptions, SqlitePool},
};
use std::str::FromStr;
//Handle connection through environment variables. Provide URI as parameter. Use dotenv here?

struct InputData{
    method: String, //GET|PUT|POST|DELETE , reject all else
    body: String,   //{location, forecast_duration, ...}
}
struct OutputData {
    response: i32,  //review: 1xx - 5xx
    body: String,   //{requested resource}
}
//Validation is handled server-side. Only valid responses *should* reach this point, but check to be safe.
//Better to consider body as JSON? Break down even further at server?

//weather
fn create_weather(input: InputData) -> OutputData {

}
fn read_weather(input: InputData) -> OutputData {

}
fn update_weather(input: InputData) -> OutputData {

}
fn delete_weather(input: InputData) -> OutputData {

}

//logging: log query, time, date

fn log_query() {

}


//configuration: Server must connect to database on startup.
fn init_server_connection() {
    //Ensure only one connection to database exists.
    //Hard-coded for testing. Keep as .env variable on completion?
    let opts = SqliteConnectionOptions::from_str("sqlite:ws_data.db")?;
    let pool = SqlitePool::connect_with(opts).await?;

    //CRUD Operations tests.
    let query_test = sqlx::query!(
        
    );
}
