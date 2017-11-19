use request::Request;
use response::Response;

pub type Handler = fn(&mut Request) -> Response;

pub struct Route {
    pub path: String,
    pub handler: Handler,
}

impl Clone for Route {
    fn clone(&self) -> Route {
        Route { path: self.path.clone(), handler: self.handler }
    }
}

impl Route {
    pub fn new(path: &str, handler: Handler) -> Route {
        Route {
            path: String::from(path),
            handler: handler,
        }
    }

    pub fn handle(&self, request: &mut Request) -> Response {
        (self.handler)(request)
    }
}

#[derive(Clone)]
pub struct Router {
    routes: Vec<Box<Route>>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(Box::new(route));
    }

    pub fn get_route(&self, request: &Request) -> Route {
        (*self.routes.first().unwrap().clone())
    }
}
