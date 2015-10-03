#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter};
extern crate postgres;
extern crate postgres_array;
extern crate chrono;
use std::collections::HashMap;
use postgres::{Connection, SslMode};
mod backend;
//mod routing;
fn main(){
            let mut server = Nickel::new();
            let conn = Connection::connect("postgres://postgres@localhost/silmukka", &SslMode::None).unwrap();
            let mut reititys_alustus: Vec<nickel::router::router::Router> = Vec::new();

            for _ in 0..5{
                reititys_alustus.push(server.router()));
            } 
  //          let mut routers = routing::routers(); 
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
        let conn = Connection::connect("postgres://postgres@localhost/testi", &SslMode::None)
                        .unwrap();
        println!("yhteys luotu");
        let admin = backend::luo_kayttaja("Leo Lahti".to_string(), &conn);
        println!("{}", admin);
}
#[test]
fn tapahtuma(){
        let conn = Connection::connect("postgres://postgres@localhost/testi", &SslMode::None).unwrap();
        let tapahtuma = backend::luo_tapahtuma("redu".to_string(), 1, &conn);
        println!("{}", tapahtuma);
}
