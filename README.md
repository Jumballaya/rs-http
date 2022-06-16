# Simple HTTP Server

Http Server modeled off of ExpressJS

## Example

```rust
use http::{
    app::App,
    request::Request,
    response::Response,
    router::Router,
};

fn main() -> Result<(), std::io::Error> {
    let mut app = App::new();
    let mut main_router = Router::new("/");

    main_router.get("/", index_route);
    app.add_router(main_router);
    app.listen("127.0.0.1", 3000)
}

fn index_route(req: &Request, res: &mut Response) {
    println!("Index route: {} with Method: {}", req.route(), req.method());
    res.set_header("Content-Type", "application/json");
    res.set_status(200);
    res.set_body("{\"page\": \"index\"}");
}
```