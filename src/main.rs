#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter};
extern crate postgres;
extern crate rand;
extern crate postgres_array;
extern crate chrono;
use std::collections::HashMap;
use postgres::{Connection, SslMode};
mod backend;
mod routing;
fn main(){
            let mut server = Nickel::new();
            let conn = Connection::connect("postgres://postgres:goldmine@localhost/silmukka", &SslMode::None).unwrap();
            let routers = routing::routers(&conn); 
            for router in routers{
                server.utilize(router);
            }
            server.listen("127.0.0.1:6768");
}
#[test]
fn database(){
        let conn = Connection::connect("postgres://postgres@localhost/testi", &SslMode::None).unwrap();
            conn.execute("INSERT INTO testi (name) VALUES ($1)",
                             &[&"me"]).unwrap();
}
#[test]
fn admin(){
        let conn = Connection::connect("postgres://postgres@localhost/silmukka", &SslMode::None)
                        .unwrap();
        println!("yhteys luotu");
 //       let admin = backend::luo_kayttaja("Leo Lahti".to_string(), &conn);
  //      println!("{}", admin);
}
#[test]
fn tapahtuma(){
        let conn = Connection::connect("postgres://postgres@localhost/silmukka", &SslMode::None).unwrap();
        let _ = backend::luo_kayttaja("Leo Lahti".to_string(), "Leo".to_string(), "TearsInTheRain".to_string(), &conn);
        
}
