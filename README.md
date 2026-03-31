***rusty_weather: A Web Service for the US National Weather Service API***
A rust crate made as an exercise in learning the rust language, client-server RESTful practices, and API endpoints.

As there is no intuitive way to request the weather of a city from the NOAA API Web Service itself (the user must know the weather forecast office of a location and a set of coordinates corresponding to requested location) this tool simplifies the process by enabling lookup of forecast by city and state alone.

City data from simplemaps is converted to sqlite database using python scripts. (pandas, sqlite3)
Rust server hosts sqlite database, handles queries, and listens for HTTP requests.
Clients can send HTTP requests and receive responses as JSON.


United States city database sourced from: https://simplemaps.com/data/us-cities
