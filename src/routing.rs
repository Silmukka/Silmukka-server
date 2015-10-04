use nickel;
use nickel::{Nickel, HttpRouter};
use nickel::router::router::Router;
use postgres::Connection;
use std::collections::HashMap;
use std::io::Read;
use backend;
use std::sync::mpsc;
use std::thread;
use chrono;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
static mut CURRENT: i32 = 0;
fn home()->Router
{
        let mut router = Nickel::router();
        router.get("/", middleware! {|_ , response|
            let mut data = HashMap::new();
                let mut string: String = "".to_string();
                let mut vec = backend::suosituimmat();
                for tapahtuma in &vec{
                        string=string+"<tr><td>"+&tapahtuma+"</td></tr>"    
                }
                data.insert("suosituimmat", string);
                return response.render("assets/index.html", &data);
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
fn paskahash(stri: String)->String
{
    stri
}
fn valid(conn: &Connection)->Router
{
    let mut vec = backend::hae_kayttajan_tunnus(conn);
    let mut router = Nickel::router();
    router.post("/valid", middleware!{|req, res|
        let mut form_data = String::new();
            req.origin.read_to_string(&mut form_data).unwrap();
            let mut usr = String::new();
            let mut pswd = String::new();
            if form_data.len() == 0{
                panic!("usr was zero");
            }
            form_data.remove(0);
            form_data.remove(0);
            for i in form_data.clone().chars(){
                if i == '&'{
                    break;
                }
                usr.push(form_data.remove(0));
            }
            let mut suola = String::new();
            let mut verrattava = String::new();
            for i in vec.clone(){
                    if usr == i.0{
                        suola = i.1;
                        verrattava = i.2;
                        break;
                    }
            }
            form_data.remove(0);
            form_data.remove(0);
            form_data.remove(0);
            for i in form_data.chars(){
                pswd.push(i);    
            }
            let mut data = HashMap::new();
            data.insert("To be or to not to be", "Hamlet");
            if paskahash((pswd).to_string()) != verrattava{
                return res.render("assets/surkea_yritys.html", &data);
            }
            return res.render("assets/valid.html", &data);
    });

    return router;

}
fn tapahtuman_luonti_handle()->Vec<Router> 
{
    let mut r1 = Nickel::router();
    let mut r3 = Nickel::router();
    r1.get("/create_event", middleware!{| _, response|

        let mut data = HashMap::new();
        data.insert("name", "data");
        return response.render("assets/luo_tapahtuma.html", &data);
    });
    let mut r2 = Nickel::router();
    r2.get("/event_process", middleware!{| _, response|
        let mut data = HashMap::new();
        let l = thread::spawn(move||{unsafe{CURRENT = backend::luo_tapahtuma("tapahtuma".to_string(), 1)}; });
        data.insert("name", "data");
        l.join();
        return response.render("assets/tapahtuman_luonti.html", &data);
    });
    let mut ra = vec![r1, r2];
    r3.post("/edit_event", middleware!{| req, response|
        let mut form_data = String::new();
        req.origin.read_to_string(&mut form_data).unwrap();
       
        form_data.remove(0);
        let mut name = String::new();
        let mut kuvaus = String::new();
        let mut list = String::new();
        let mut listaa = vec![name, kuvaus, list];
        let mut lista: Vec<String> = Vec::new();
        for mut i in  listaa.clone(){
            
        for chara in form_data.clone().chars(){
            if chara == '&'{
            //    form_data.remove(0);
                break;
            }
            else if chara == '=' || chara == '+'{
             //   form_data.remove(0)
            }
            else{
                i.push(chara);
            }
            }
            lista.push(i);
        }
        let mut nimi  = lista.remove(0); 
        let mut kuvaus = lista.remove(0);
        let mut lista_ = vec![Some(lista.remove(0))];
        let mut data = HashMap::new();
        data.insert("a", "b");
        backend::muokkaa_tapahtuma(backend::Tapahtuma{id: unsafe{CURRENT}, nimi: nimi, lista: lista_, 
            osallistujat: vec![Some(0)], adminit: 
            vec![Some(1)],
                kuvaus: kuvaus});
        return response.render("assets/valid.html", &data);

    });
    ra.push(r3);
    return ra;
}

pub fn routers(conn: &Connection)->Vec<Router>
{
    let mut routes: Vec<Router> = Vec::new();
    routes.push(home());
    routes.push(login());
    routes.push(valid(&conn));
    for i in tapahtuman_luonti_handle(){
        routes.push(i);
    }
    return routes;

}
