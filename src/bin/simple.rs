use http::{app::App, request::Request, response::Response, router::Router};
use std::thread;

fn main() -> Result<(), std::io::Error> {
    let mut app = App::new();
    let mut main_router = Router::new("/");

    main_router.get("/", index_route);
    main_router.get("/sleep", sleep_route);
    main_router.get("/user/:user_id/post/:post_id", user_post);

    app.add_router(main_router);
    app.listen("127.0.0.1", 3000)
}

fn index_route(req: &Request, res: &mut Response) {
    println!("Index route: {} with Method: {}", req.route(), req.method());

    match req.get_header("Content-Type") {
        Some(s) => println!("{}", s),
        None => {}
    }
    println!("Body: \n{}\n", String::from_utf8_lossy(req.body()));
    res.set_header("Content-Type", "application/json");
    res.set_status(200);
    res.set_body("{\"page\": \"index\"}");
}

fn sleep_route(req: &Request, res: &mut Response) {
    println!("Index route: {} with Method: {}", req.route(), req.method());
    res.set_header("Content-Type", "application/json");
    res.set_status(200);
    thread::sleep(std::time::Duration::from_secs(5));
    res.set_body("{\"page\": \"sleep\"}");
}

fn user_post(req: &Request, res: &mut Response) {
    println!(
        "User Profile route: {} with Method: {}",
        req.route(),
        req.method()
    );
    res.set_header("Content-Type", "application/json");
    res.set_status(200);
    let user_id = req.get_url_param("user_id").unwrap();
    let post_id = req.get_url_param("post_id").unwrap();
    res.set_body(&format!(
        "{{\"page\": \"post\", \"post\": {{ \"user\": \"{}\", \"id\": \"{}\", \"length\": \"length\" }} }}",
        user_id, post_id
    ));
}
