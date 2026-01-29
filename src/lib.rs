//Standard set of instructions for server to execute business logic.
//Clients, e.g. CLI, android widget, web app, may run platform-dependent instructions.
//Standardize at server-level, i.e. Receive same objects, send same objects.
//Send platform-dependent packets as necessary.

//Example Exchange:
//Client
//platform|request_type|content
//Server
//acknowledgement|response_type|content

//We want to handle data in standardized packets.
//Isolate data layers.

//Should have access to buffer to perform actions.

//Handle fetching of external data here.


struct WeatherReport {
    description: String,
    temperature: i8,
    icon: String, //path to a resource | ASCII?
}
enum Format {
    CLI,
    Web,
    Android,
}

//Database functions
mod poll_data {

}

//Read and write http along data stream
mod serving {
    use http::{Request, Response};
    
    //review how requests and responses are handled
    fn respond(req: Request<()>) -> http::Result<Response<()>> {
        //access fields of req
        
        Ok(Response::default()) //debug
        
    }
    fn send_to_client(res: Response<()>) {
        //writes to connection specified
    }
}


//ABOVE: Private: Module tree, Internal functions
//BELOW: Public : Accessible actions

//Adhere to REST Standards
//GET:      200: return xml/JSON resource | 400 | 404
//POST:     201: create new resource, return location header
//PUT:      200: update existing resource / create new resource (full replacement)
//PATCH     200: partial update existing resource (replaces specific fields)
//DELETE    200: deletes resource by URL


