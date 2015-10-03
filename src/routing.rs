use nickel;
use nickel::{Nickel, HttpRouter};
use nickel::router::router::Router;
use postgres::Connection;
use std::collections::HashMap;
use std::io::Read;
fn home()->Router
{
        let mut router = Nickel::router();
        router.get("/", middleware! {
                "Silmukka"
            });
        return router;
}
fn login()->Router
{
    let mut router = Nickel::router();
    router.get("/login", middleware!{ |_, response|
            let mut data = HashMap::new();
            data.insert("name", "user");
            return response.render("assets/login.html", &data);
    });
    return router;
}
fn valid()->Router
{
    let mut router = Nickel::router();
    router.post("/valid", middleware!{  |req, res|
        let mut form_data = String::new();
            req.origin.read_to_string(&mut form_data).unwrap();
            
            println!("{}", form_data);
            let mut data = HashMap::new();
            data.insert("To be or to not to be", "Hamlet");
            return res.render("assets/valid.html", &data)
    });
    return router;

}
pub fn routers(conn: &Connection)->Vec<Router>
{
    let mut routes: Vec<Router> = Vec::new();
    routes.push(home());
    routes.push(login());
    routes.push(valid());
    return routes;

}
