//! Basic router
//! - new: Create a router with API url
//! - add_route: Add API paths to the routes mapping
//! - get: returns Response with details like status code, message, body.
use std::collections::HashMap;

struct Request<'rq> {
    url: &'rq str,
    path: &'rq str,
    header: &'rq str,
    body: &'rq str,
}

#[derive(Debug, PartialEq, Clone)]
struct Response {
    status_code: u16,
    message: &'static str,
    body: String,
}

#[derive(Debug)]
struct Router {
    url: &'static str,
    routes: HashMap<&'static str, Response>,
}

impl Router {
    fn new(url: &'static str) -> Self {
        Self {
            url,
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, path: &'static str, response: Response) {
        self.routes.insert(path, response);
    }

    fn get(&self, path: &'static str) -> Response {
        match self.routes.get(path) {
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
    fn it_works() {
        let url = "https://cmc.com/api";
        let mut router = Router::new(url);

        router.add_route("/", api_status());
        router.add_route("/about", about());

        assert_eq!(router.get("/patients"), not_found());

        dbg!(router);
    }
}
