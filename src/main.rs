use std::collection::HashMap;
use warp::Filter;
use sqlx::sqlite::SqlitePoolOptions;
use serde::{Deserialize, Serialize};
use ws_api::{ db_access, weather_api };
//Initialize connection to database.
//Listen for client connection attempts.
//Close on process exit.


//https://github.com/Spades-Ace/RawRustServer/blob/main/src/main.rs
//mirror implementation of RawRustServer: handling of connections and http protocol
//modify to handle async tokio constraints
//https://tokio.rs/tokio/tutorial 
//use hyper/warp
//https://users.rust-lang.org/t/processing-http-requests-from-a-tokio-tcpstream/48464



//API: MODELS, HANDLERS, AND ROUTES

//TODO: WARP: Define model: shape of data
    //requestForecast
    //requestForecastWeekly
    //requestForecastGridData
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Request {
    
}
//TODO: WARP: Define handlers: process http request
    //Database Access + NOAA API Call + Cache
    //Cached Location + NOAA API Call
async fn get_forecast_from_db() {
    //db access
    //API call
    //add location to server cache
}
async fn get_forecast_cached() {
    //location requested prior, is on cache (HashMap?)
    //API call
}
//TODO: WARP: Define routes: API endpoints
    //forecast
    //forecastWeekly
    //forecastGridData
fn routes () {

}
fn get_forecast() {

}
fn get_forecast_weekly() {

}
fn get_forecast_griddata() {

}
//after above are defined, understand how warp filters work

#[tokio::main]
async fn main() {
    //startup:database connection
    let pool = SqlitePoolOptions::new()
        .max_connections(3)
        .connect("sqlite://data/uscities.db")
        .await?;
    let mut location_cache - HashMap::new(); //previously accessed locations

    warp::server(routes).run([127, 0, 0, 1], 8080).await;
}
