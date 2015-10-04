//Sisältää henkilö-luokan.
use postgres::{Connection, SslMode};
use postgres_array::array::Array;
use chrono::NaiveDate;
use rand;
fn connect()->Connection{
    Connection::connect("postgres://postgres@localhost/silmukka", &SslMode::None).unwrap()
}
#[derive(Clone, Debug)]
pub struct Tapahtuma{
    pub id: i32,
    pub nimi: String,
    pub osallistujat: Vec<Option<i32>>, //id, arvo. arvoja 2 osallistuja ja johtaja.
    pub adminit: Vec<Option<i32>>,
    pub kuvaus: String,
    pub lista: Vec<Option<String>>,
}
pub struct Kayttaja{
    pub id: i32,
    pub nimi: String,
    pub osallistuu: Vec<Option<i32>>,
    pub admin: Vec<Option<i32>>
}
pub fn suosituimmat()->Vec<String>
{
        let mut vec: Vec<String> = Vec::new();
        let mut conn = connect();
        let stmt = conn.prepare("SELECT nimi FROM tapahtuma ORDER BY maara DESC limit 1").unwrap(); 
        for row in stmt.query(&[]).unwrap(){
            vec.push(row.get(0));   
        }
        return vec;
}
pub fn luo_tapahtuma(nimi: String, luoja: i32)->i32 //palauttaa tapahtuman IDn
{
        let mut conn = connect();
        let osallistujat: Vec<Option<i32>> = Vec::new();        
        conn.execute("INSERT INTO tapahtuma (nimi, osallistujat, adminit, maara) VALUES ($1, $2, $3, $4 )", 
            &[&nimi,  &Array::from_vec(osallistujat, 1), &Array::from_vec(vec![Some(luoja)], 1), &1]).unwrap();
        let stmt = conn.prepare("SELECT id FROM tapahtuma WHERE nimi = $1 ORDER BY id DESC limit 1", ).unwrap();
        let mut palautus: i32 = 0;
        for row in stmt.query(&[&nimi]).unwrap() {
             palautus = row.get(0);
        }
        return palautus;
}
fn vec_from_array<T>(array:  Array<Option<T>>)->Vec<T>{
            let mut vec: Vec<T> = Vec::new();
//            let mut iter = array.iter();
            for object in array{
                vec.push(object.unwrap());
            }
            return vec;
}
pub fn hae_tapahtuma_id(id: i32, conn: &Connection)->Tapahtuma //Hakee tapahtuman ID:n perusteella
{
    let mut vec: Vec<Tapahtuma> = Vec::new();
    let stmt = conn.prepare("SELECT * FROM tapahtuma WHERE id = $1").unwrap();
     for row in stmt.query(&[&id]).unwrap(){
           vec.push(Tapahtuma{
                id: row.get(0),
                nimi: row.get(1),
                osallistujat: vec_from_array(row.get(2)),
                adminit: vec_from_array(row.get(3)),
                kuvaus: row.get(4),
                lista: vec_from_array(row.get(5)),
            });
    }
     vec.pop().unwrap()
    
}
pub fn muokkaa_tapahtuma(tapahtuma: Tapahtuma)
{
    let mut conn = connect();
    let mut luku = (tapahtuma.clone().adminit.len()+tapahtuma.clone().osallistujat.len()) as i32;
    let stmt = conn.prepare("UPDATE tapahtuma SET nimi = $1, osallistujat = $2, adminit = $3,
                            kuvaus = $4, lista = $5,  maara = $6 WHERE id = $7").unwrap();
    let paivitetty = stmt.execute(&[&tapahtuma.nimi,  &Array::from_vec(tapahtuma.osallistujat, 1), &Array::from_vec(tapahtuma.adminit, 1),
                    &tapahtuma.kuvaus,  &Array::from_vec(tapahtuma.lista, 1), &luku, &tapahtuma.id]).unwrap();
    let ss = conn.execute("DELETE FROM tapahtuma WHERE maara = 1", &[]).unwrap();
}
pub fn luo_kayttaja(nimi: String, us: String, sala: String, conn: &Connection)->i32
{
    let mut suola = String::new();
    for _ in 0..30{
        suola.push(rand::random::<char>());
    }
    let vec: Vec<Option<i32>> = Vec::new();
    conn.execute("INSERT INTO kayttaja (kayttajanimi, suola, salasana, nimi, osallistuu, admin) VALUES ($1, $2, $3, $4, $5, $6)", 
                 &[&us, &suola, &(sala+&suola), &nimi,  &Array::from_vec(vec.clone(), 1), 
                 &Array::from_vec(vec, 1)]).unwrap();
    let stmt = conn.prepare("SELECT id FROM kayttaja WHERE nimi = $1 ORDER BY id DESC limit 1").unwrap();
    let mut palautus: i32 = 0;
    for row in stmt.query(&[&nimi]).unwrap(){
        palautus = row.get(0);
    }
    return palautus;
}
pub fn hae_kayttajan_tunnus(conn: &Connection)->Vec<(String, String, String)>
{
    let mut tple: Vec<(String, String, String)> = Vec::new();
    let stmt = conn.prepare("select * from kayttaja").unwrap();
    for row in stmt.query(&[]).unwrap(){
        tple.push((row.get(1), row.get(2), row.get(3)));
    } 
    return tple;
}
/*pub fn hae_kayttaja_id(id: i32, conn: &Connection)->Kayttaja
{
    let mut vec: Vec<Kayttaja> = Vec::new();
    let stmt = conn.prepare("SELECT * FROM kayttaja WHERE id = $i").unwrap();
    for row in stmt.query(&[&id]){
        vec.push(Kayttaja{
        id: row.get(0),
        nimi: row.get(1),
        osallistuu: vec_from_array(row.get(2)),
        admin: vec_from_array(row.get(3))
        
        });
    }
    vec.pop().unwrap()
}*/
