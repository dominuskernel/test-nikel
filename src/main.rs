#[macro_use] extern crate nickel;
extern crate hyper;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use hyper::header::{AccessControlAllowOrigin, AccessControlAllowHeaders};

fn enable_cors<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.set(AccessControlAllowOrigin::Any);
    res.set(AccessControlAllowHeaders(vec![
        "Origin".into(),
        "X-Requested-With".into(),
        "Content-Type".into(),
        "Accept".into(),
    ]));
    res.next_middleware()
}

fn main() {
    let mut server = Nickel::new();
    server.utilize(enable_cors);
    server.get("/", middleware! { |request,response|
        format!("<h1>WELCOME TO API</h1>")
    });

    server.get("/user/:userid", middleware! { |request,response|
        println!("User id: {:?}\n", request.param("userid"));
        format!("This id user: {:?}", request.param("userid"))
    });

    server.get("user/login", middleware! { |request, response|
        println!("The login was succesfull: username {:?}, password {:?}", request.param("userName"), request.param("password"))
    });

    server.listen("127.0.0.1:6767");
}
