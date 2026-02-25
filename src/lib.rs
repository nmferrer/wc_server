//3-Stage Request
//Prepare Data  : Read user input, query database for necessary API data
//Execute Query : Send API request. Handle Response.
//Receive       : Server sends response to user as JSON.

//running as a separate binary for testing
//integrate as library for server?

enum ForecastType {
    Forecast,
    ForecastHourly,
    ForecastGridData,
}
struct Input {
    city: String,
    state: String,
    forecast: ForecastType,
}
struct Coordinates {
    latitude: f32,
    longitude: f32,
}
//struct Request {}
//struct Response {}

mod db_access {
    use sqlx::sqlite:: { SqlitePool };
    use crate::Coordinates;

    //OKOK: Given (City, State) -> Retrieve (lat, lng)
    pub async fn prepare_api_call(connection: &SqlitePool, input: &crate::Input) -> anyhow::Result<crate::Coordinates> {
    //prepared sql query: Query on Table cities where city = city
     let city_data: (f32, f32) = sqlx::query_as( //TODO: Cleanup: map to struct using #[derive(FromRow)]
            "SELECT lat, lng
            FROM city
            WHERE city = $1 AND (state_id = $2 OR state_name = $2)
            LIMIT 1",
        )
            .bind(&input.city).bind(&input.state)
            .fetch_one(connection)
            .await?;
        Ok(Coordinates {latitude: city_data.0, longitude: city_data.1})
    }

    pub async fn check_cache() {
        //returns boolean whether attempted query is already in database
    }
}
mod weather_api {
    use reqwest::header::USER_AGENT;
    use serde_json::Value;
    use crate::ForecastType;
    //OKOK: Given (longitude, lati=tude) -> Retrive (wfo, grid_x, grid_y)
    pub async fn fetch_gridpoint_wfo(client: &reqwest::Client, input: &crate::Input, coordinates: crate::Coordinates) -> anyhow::Result<String> {
        //can be skipped if location cached on local database
        let (lat, lng) = (coordinates.latitude, coordinates.longitude);
        let url = format!("https://api.weather.gov/points/{},{}", lat, lng);
        let res = client.get(url).header(USER_AGENT, "rusty_weather").send().await?;
        let body = res.text().await?;
        
        //PARSE BODY: Relevant Fields: { forecast, forecastHourly, forecastGridData }
        //can deserialize as strongly-typed json by .json() call
        //loosely-typed example
        let v: Value = serde_json::from_str(&body)?;
        match &input.forecast { //return here
            ForecastType::Forecast => 
                Ok(String::from(v["properties"]["forecast"].as_str().unwrap())),
            ForecastType::ForecastHourly => 
                Ok(String::from(v["properties"]["forecastHourly"].as_str().unwrap())),
            ForecastType::ForecastGridData => 
                Ok(String::from(v["properties"]["forecastGridData"].as_str().unwrap())),
        } //TODO: Cache onto db to reduce redundant database queries
    }
    pub async fn fetch_forecast(client: &reqwest::Client, forecast_endpoint: String) -> anyhow::Result<()> {
        //API endpoint might be provided by above call, review docs
        let res = client.get(forecast_endpoint).header(USER_AGENT, "rusty_weather").send().await?;
        let body = res.text().await?;

        //TODO: Different ways to parse based on {forecast|forecastHourly|forecastGridData}
        let v: Value = serde_json::from_str(&body)?;
        
        //println!("{:#?}", v["properties"]["periods"].as_array().unwrap()); //DEBUG: See all fields
        for p in v["properties"]["periods"].as_array().unwrap() {
            let p_data = p.as_object().unwrap();
            println!("{}: {}", p_data["name"].as_str().unwrap(), p_data["detailedForecast"].as_str().unwrap());
            //{attribute:String : value:String} //FORMAT HERE, trim
        }
        //OKOK: PARSE: forecastHourly
        //OKOK: PARSE: forecastGridData
        Ok(())
    }
}


//OKOK: Confirm working connection
//TODO: Confirm queries work as intended
//OKOK: Confirm API calls execute properly. Handle requests and responses.
//TODO: Error handling. Write tests.
//Upon completion, rewrite as separate library to be run on server.


/* 
//HOLDOVER FROM TESTING AS A BINARY
//NOW TESTING IN MAIN SERVER BINARY
use sqlx::sqlite::{ SqlitePoolOptions };
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Database Connection
    let pool = SqlitePoolOptions::new()
        .max_connections(3)
        .connect("sqlite://data/uscities.db")
        .await?; //localDB, env variables? 
    //HTTP Client
    let client = reqwest::Client::builder()
        .build()
        .expect("client built");

    let input0 = Input {
                city:String::from("Los Angeles"),
                state:String::from("CA"),
                forecast:ForecastType::Forecast,
    };
    let input1 = Input {
                city:String::from("San Francisco"),
                state:String::from("California"),
                forecast:ForecastType::ForecastHourly,
    };
    let input2 = Input {
                city:String::from("Sacramento"), 
                state:String::from("CA"), 
                forecast:ForecastType::ForecastGridData,
    };
    //DB Query: city -> long_x,lat_y
    let c0 = db_access::prepare_api_call(&pool, &input0).await?;
    let c1 = db_access::prepare_api_call(&pool, &input1).await?;
    let c2 = db_access::prepare_api_call(&pool, &input2).await?;

    //API Call: long_x,lat_y -> gridpoint, wfo. Cache as needed.
    let gw0 = weather_api::fetch_gridpoint_wfo(&client, &input0, c0).await?;
    let gw1 = weather_api::fetch_gridpoint_wfo(&client, &input1, c1).await?;
    let gw2 = weather_api::fetch_gridpoint_wfo(&client, &input2, c2).await?;


    //println!("{}", gw0);
    //println!("{}", gw1);
    //println!("{}", gw2);

    weather_api::fetch_forecast(&client, gw0).await?;

    //API Call: gridpoint, wfo, forecastType -> Forecast. Package output for Response body.
    Ok(())
}

*/
