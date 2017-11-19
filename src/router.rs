use http_request::HTTPRequest;

pub struct Route {
    pub path: String,
    pub handler: fn(&mut HTTPRequest) -> String,
}

impl Clone for Route {
    fn clone(&self) -> Route {
        Route { path: self.path.clone(), handler: self.handler }
    }
}

impl Route {
    pub fn handle(&self, request: &mut HTTPRequest) -> String {
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

    pub fn get_route(&self, request: &HTTPRequest) -> Route {
        (*self.routes.first().unwrap().clone())
    }
}
