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

enum Format {
    CLI,
    Web,
    Android,
}
struct Request {
    format: Format,
    body: String, //Placeholder, should contain content from client.
}
struct Response {
    body: String, //Response should be consistent.
}

//consider impl for struct functions

mod receive {
    use crate::Format;
    use crate::Request;
    use crate::Response;

    fn handle_request(req: &Request) -> Response {
        //Unpacks request and executes appropriate API calls
        //returns packet ready for client.
        
        let mut result = Response{body: String::from("200")};
        match req.format {
            Format::CLI => result = Response{body: String::from("Success")}, //unpack and execute
                      _ => result = Response{body: String::from("Failure")}, //error handling
        }
        return result
    }
}

mod respond {
    //fn send_response() -> {}
}

//ABOVE: Private: Module tree, Internal functions
//BELOW: Public : Accessible actions

//Adhere to REST Standards
//GET:      200: return xml/JSON resource | 400 | 404
//POST:     201: create new resource, return location header
//PUT:      200: update existing resource / create new resource (full replacement)
//PATCH     200: partial update existing resource (replaces specific fields)
//DELETE    200: deletes resource by URL


