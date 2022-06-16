use crate::request::Request;
use crate::response::Response;
use crate::router::Router;
use crate::thread_pool::ThreadPool;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    sync::Mutex,
};

pub struct App {
    routers: Vec<Router>,
    thread_pool: ThreadPool,
}

impl App {
    pub fn new() -> Self {
        let thread_pool = match ThreadPool::new(10) {
            Ok(pool) => pool,
            Err(e) => {
                println!("Application error: {}", e);
                std::process::exit(1);
            }
        };
        Self {
            routers: Vec::<Router>::new(),
            thread_pool,
        }
    }

    pub fn add_router(&mut self, router: Router) {
        self.routers.push(router);
    }

    pub fn listen(&self, host: &str, port: usize) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(format!("{}:{}", host, port))?;

        for stream in listener.incoming() {
            let mut stream = stream?;

            //
            //  Get an async-safe, wrapped, clone of the routers Vec
            //
            let routers = Arc::new(Mutex::new(self.routers.to_vec()));

            match self.thread_pool.execute(move || {
                //
                //  Prepare the socket, request/response and the data needed to find the route
                //
                let incoming = read_incoming(&mut stream).unwrap();
                let mut req = Request::new(incoming);
                let mut res = Response::new();
                let method = req.method();
                let route = req.route();

                //
                //  Search for the correct route and any route params, otherwise return the default 404 route
                //
                let (handler, url_params) = routers
                    .lock()
                    .unwrap()
                    .iter()
                    .find_map(|r| r.match_handler(method, route))
                    .unwrap_or((not_found_handler, None));

                req.set_url_params(url_params);
                //
                //  Fire off the handler and then write to the client
                //
                //  @TODO: The handler should actually be a iterator of RouteHandlers that the req/res are fed into
                //         until an error is thrown or the iterator is over
                //
                handler(&req, &mut res);

                stream.write(&res.build_response()).unwrap();
                stream.flush().unwrap();
            }) {
                Err(e) => {
                    println!("Application error: {}", e);
                }
                _ => {}
            };
        }
        Ok(())
    }
}

fn not_found_handler(_: &Request, res: &mut Response) {
    res.set_header("Content-Type", "application/json");
    res.set_status(404);
    res.set_body("{\"error\": \"not found\"}");
}

fn read_incoming(socket: &mut TcpStream) -> Result<[u8; 1024], std::io::Error> {
    let mut buf: [u8; 1024] = [0; 1024];
    socket.read(&mut buf)?;
    Ok(buf)
}
