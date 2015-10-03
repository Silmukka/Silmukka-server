#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter};
extern crate postgres;
extern crate postgres_array;
extern crate chrono;
use postgres::{Connection, SslMode};
mod backend;
fn main(){
            let mut server = Nickel::new();
            server.get("**", middleware!("Silmukka-projekti, tarvitaan HTML"));
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
