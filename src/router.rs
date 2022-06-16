use std::collections::HashMap;

use crate::{
    matcher::RouteMatcher,
    request::{HttpMethod, Request},
    response::Response,
};

type RouteHandler = fn(&Request, &mut Response);

#[derive(Clone)]
pub struct Endpoint {
    handler: RouteHandler,
    matcher: RouteMatcher,
}

impl Endpoint {
    pub fn new(path: String, handler: RouteHandler) -> Self {
        let matcher = RouteMatcher::new(&path);
        Self { handler, matcher }
    }

    pub fn matches(&self, path: &str) -> Option<HashMap<String, String>> {
        let matches = self.matcher.matches(path);
        matches
    }
}

#[derive(Clone)]
pub struct Router {
    path: String,
    routes: HashMap<HttpMethod, Vec<Endpoint>>,
}

impl Router {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            routes: HashMap::<HttpMethod, Vec<Endpoint>>::new(),
        }
    }

    pub fn get(&mut self, name: &str, handler: RouteHandler) {
        self.add_route(name, HttpMethod::Get, handler);
    }

    pub fn post(&mut self, name: &str, handler: RouteHandler) {
        self.add_route(name, HttpMethod::Post, handler);
    }

    pub fn put(&mut self, name: &str, handler: RouteHandler) {
        self.add_route(name, HttpMethod::Put, handler);
    }

    pub fn patch(&mut self, name: &str, handler: RouteHandler) {
        self.add_route(name, HttpMethod::Patch, handler);
    }

    pub fn delete(&mut self, name: &str, handler: RouteHandler) {
        self.add_route(name, HttpMethod::Delete, handler);
    }

    pub fn add_route(&mut self, name: &str, method: HttpMethod, handler: RouteHandler) {
        self.routes
            .entry(method)
            .or_insert_with(Vec::<Endpoint>::new)
            .push(Endpoint::new(Self::format_path(&self.path, name), handler));
    }

    pub fn match_handler(
        &self,
        method: &HttpMethod,
        route: &str,
    ) -> Option<(RouteHandler, Option<HashMap<String, String>>)> {
        if let Some(handlers) = self.routes.get(method) {
            if let Some(handler) = handlers
                .iter()
                .filter(|endpoint| match endpoint.matches(&route) {
                    Some(_) => true,
                    _ => false,
                })
                .collect::<Vec<&Endpoint>>()
                .first()
            {
                Some((handler.handler, handler.matcher.matches(&route)))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn format_path(base: &str, path: &str) -> String {
        if base.ends_with("/") && path.starts_with("/") {
            format!("{}{}", base.trim_end_matches("/"), path)
        } else {
            format!("{}{}", base, path)
        }
    }
}
