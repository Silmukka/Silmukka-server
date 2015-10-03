//Sisältää henkilö-luokan.
use postgres::{Connection, SslMode};
use postgres_array::array::Array;
use chrono::NaiveDate;
pub struct Tapahtuma{
    pub id: i32,
    pub nimi: String,
    pub osallistujat: Vec<Option<i32>>, //id, arvo. arvoja 2 osallistuja ja johtaja.
    pub adminit: Vec<Option<i32>>,
    pub kuvaus: String,
    pub lista: Vec<Option<String>>,
    pub pvm: NaiveDate,
}
pub struct Kayttaja{
    pub id: i32,
    pub nimi: String,
    pub osallistuu: Vec<Option<i32>>,
    pub admin: Vec<Option<i32>>
}
pub fn luo_tapahtuma(nimi: String, luoja: i32, conn: &Connection)->i32 //palauttaa tapahtuman IDn
{
        let osallistujat: Vec<Option<i32>> = Vec::new();        
        conn.execute("INSERT INTO tapahtuma (nimi, osallistujat, adminit) VALUES ($1, $2, $3 )", 
            &[&nimi,  &Array::from_vec(osallistujat, 1), &Array::from_vec(vec![Some(luoja)], 1)]).unwrap();
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
                pvm: row.get(6)
            });
    }
     vec.pop().unwrap()
    
}
pub fn muokkaa_tapahtuma(tapahtuma: Tapahtuma, conn: &Connection)
{
    let stmt = conn.prepare("UPDATE tapahtuma SET nimi = $1, osallistujat = $2, adminit = $3,
                            kuvaus = $4, lista = $5, aika = $6 WHERE id = $7").unwrap();
    let paivitetty = stmt.execute(&[&tapahtuma.nimi,  &Array::from_vec(tapahtuma.osallistujat, 1), &Array::from_vec(tapahtuma.adminit, 1),
                    &tapahtuma.kuvaus,  &Array::from_vec(tapahtuma.lista, 1), &tapahtuma.pvm, &tapahtuma.id]).unwrap();
    println!("päivitettiin {} riviä", paivitetty);
}
pub fn luo_kayttaja(nimi: String, conn: &Connection)->i32
{
    let vec: Vec<Option<i32>> = Vec::new();
    conn.execute("INSERT INTO kayttaja (nimi, osallistuu, admin) VALUES ($1, $2, $3)", &[&nimi,  &Array::from_vec(vec.clone(), 1), 
                 &Array::from_vec(vec, 1)]).unwrap();
    let stmt = conn.prepare("SELECT id FROM kayttaja WHERE nimi = $1 ORDER BY id DESC limit 1").unwrap();
    let mut palautus: i32 = 0;
    for row in stmt.query(&[&nimi]).unwrap(){
        palautus = row.get(0);
    }
    return palautus;
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
