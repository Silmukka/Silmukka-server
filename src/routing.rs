use nickel;
use nickel::{Nickel, HttpRouter};
use nickel::router::router::Router;
fn home()->Router
{
        let mut router = Nickel::router();
        router.get("/", middleware! {
                "Silmukka"
            });
        return router;
}

pub fn routers()->Vec<Router>
{
    let mut routes: Vec<Router> = Vec::new();
    routes.push(home());
    return routes;

}
