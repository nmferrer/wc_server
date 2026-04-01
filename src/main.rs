use warp::Filter;
use sqlx::sqlite::{ SqlitePool, SqlitePoolOptions };
use ws_api::{ Input, db_access, weather_api };


//Database Access + NOAA API Call + Cache
async fn get_forecast_from_db(recv_input: &ws_api::Input, db_connection: &SqlitePool, client: &reqwest::Client) -> Vec<String> {
    //Access db to find lat-long pair
    //Query by lat-long to find gridpoint weather forecast office 
    //Query by weather forecast office to fetch forecast[daily|weekly|gridData]
    let lat_long = db_access::prepare_api_call(db_connection, recv_input).await.unwrap(); //DB access
    let gridpoint_wfo = weather_api::fetch_gridpoint_wfo(client, recv_input, lat_long).await.unwrap();//API call
    let forecast = weather_api::fetch_forecast(client, gridpoint_wfo); //API call TODO: UNWRAP OUTPUT? SEND BACK?
    
    //add {(state,city):lat_long} to server cache
    forecast.await.expect("Forecast fetched") //JSON response of API call
}
//Cached Location + NOAA API Call
//LOCATION HAS ALREADY BEEN LOGGED BY SERVER
//SKIP DB LOOKUP, PROCEED TO weather_api
//async fn get_forecast_cached(recv_input: &ws_api::Input, db_connection: &SqlitePool, client: &reqwest::Client) {}

#[tokio::main]
async fn main() {
    //startup: database connection
    let pool = SqlitePoolOptions::new()
        .max_connections(3)
        .connect("sqlite://data/uscities.db")
        .await
        .expect("Database Connection Established");
    //startup: reqwest client to poll NOAA
    let client = reqwest::Client::builder()
        .build()
        .expect("HTTP Client Established");


    //startup: "cache" to log previously requested locations, reduces db access
    //let mut location_cache = HashMap::new(); //previously accessed locations
    //TODO: STORE K-V PAIR (city,state)-(lat_long)
    //request is made, body contains city_data and forecast type

    let forecast = //current issue: bad request to this endpoint
        warp::path("forecast")      //endpoint
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json::<ws_api::Input>()) //clone pool and client here?
        .then( move |input: Input| {
            //TODO: is this bad practice? rather, should I consider a open+close new connection instead?
            let pool_c = pool.clone(); //TODO: INSTEAD: ACQUIRE or BEGIN
            let client_c = client.clone(); //TODO: INSTEAD: NEW CLIENT?
            async move {
                println!("Received Request: {:?}", input);
                let forecast = get_forecast_from_db(&input, &pool_c, &client_c).await;
                println!("Sending response: {:?}", forecast);
                warp::reply::json(&forecast)
                //replies with requested forecast|forecast_hourly|forecast_grid_data
            }
        });
    //let forecast_hourly = warp::path("forecast_hourly").map(|| format!("forecast_hourly"));
    //let forecast_grid_data = warp::path("forecast_grid_data").map(|| format!("forecast_grid_data"));
    //CONSIDER: I can split forecast, but do I want to? Server handles action by request body.
    //CONSIDER: Additional endpoint for additional functionality, e.g. comparison, logging
    let routes = warp::get().and(
        forecast
        //.or(forecast_weekly)
        //.or(forecast_grid_data),
    );
    println!("Server running on 127.0.0.1:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
