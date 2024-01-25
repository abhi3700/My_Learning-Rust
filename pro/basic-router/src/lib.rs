//! Basic router
//! - new: Create a router with API url
//! - add_route: Add API paths to the routes mapping
//! - handle_request: returns Response with details like status code, message, body.
//!
//! Use case: TODO App shown here.
use std::collections::HashMap;

const URL: &str = "https://notemaker.app/api";

enum METHOD {
    GET,
    POST,
    DELETE,
    PUT,
}

struct Request<'rq> {
    method: METHOD,
    path: &'rq str,
    header: &'rq str,
    body: &'rq str,
}

fn get(path: &str) -> Request<'_> {
    Request {
        method: METHOD::GET,
        path,
        header: "content/type: JSON",
        body: "",
    }
}

#[derive(Debug, Clone)]
struct Response {
    status_code: u16,
    message: &'static str,
    body: String,
}

#[derive(Debug)]
struct BasicApiRouter {
    url: &'static str,
    routes: HashMap<&'static str, Response>,
}

impl BasicApiRouter {
    fn new(api_url: &'static str) -> Self {
        Self {
            url: api_url,
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, path: &'static str, response: Response) {
        self.routes.insert(path, response);
    }

    fn handle_request(&self, request: Request) -> Response {
        match self.routes.get(request.path) {
            Some(response) => response.clone(),
            None => not_found(),
        }
    }
}

fn api_status() -> Response {
    Response {
        status_code: 200,
        message: "API running successfully",
        body: "".to_string(),
    }
}

fn not_found() -> Response {
    Response {
        status_code: 404,
        message: "Not found",
        body: "".to_string(),
    }
}

fn about() -> Response {
    Response {
        status_code: 200,
        message: "OK",
        body: "CMC Vellore is a hospital in India that has world class medical facility. 
        Many Indians & NRIs have had success stories in the past. It was started back in 1900 by 
        Dr. Ida due to frequent deaths because of child births. 
        She saw her parents suffer when they tried to mitigate this catastrophe. And then she 
        started this initiative to save them for FREE."
            .to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_works() {
        let url = "https://cmc.com/api";
        let mut router = BasicApiRouter::new(url);

        router.add_route("/", api_status());
        router.add_route("/about", about());

        assert_eq!(router.handle_request(get("/")).status_code, 200);
        assert_eq!(router.handle_request(get("/about")).status_code, 200);
        assert_eq!(router.handle_request(get("/patients")).status_code, 404);

        dbg!(router);
    }
}
