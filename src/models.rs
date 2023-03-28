use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sale {
    _id: mongodb::bson::oid::ObjectId,
    key: String,
    distribution: String,
    rep: String,
    item: String,
    sale: f32,
    quantity: f32,
    uom: String,
    date: mongodb::bson::DateTime,
    customer: String,
    ship_to_name: String,
    addr1: String,
    addr2: String,
    city: String,
    state: String,
    postal: String,
    country: Option<String>,
    contract: Option<String>,
    cust_nbr: Option<String>,    
    gpo: Option<String>,
    rebate: f32,
}

impl Sale {    
    pub fn new(key: String, distribution: String, rep: String, item: String, sale: f32, quantity: f32, uom: String, date: mongodb::bson::DateTime, customer: String, ship_to_name: String, addr1: String, addr2: String, city: String, state: String, postal: String, country: Option<String>, contract: Option<String>, cust_nbr: Option<String>, gpo: Option<String>, rebate: f32) -> Self {
        Self {
            _id: mongodb::bson::oid::ObjectId::new(),
            key,
            distribution,
            rep,
            item,
            sale,
            quantity,
            uom,
            date,
            customer,
            ship_to_name,
            addr1,
            addr2,
            city,
            state,
            postal,
            country,
            contract,
            cust_nbr,
            gpo,
            rebate,
        }
    }

    pub fn print_ln(&self) {
        println!("{:?}", self);
    }

}

#[derive(Debug)]
pub struct PgSale {        
    pub id: Uuid,    
    pub key: String,
    pub distribution: String,
    pub rep: String,
    pub item: String,
    pub sale: f32,
    pub quantity: f32,
    pub uom: String,
    pub date: chrono::NaiveDateTime,
    pub customer: String,
    pub ship_to_name: String,
    pub addr1: String,
    pub addr2: String,
    pub city: String,
    pub state: String,
    pub postal: String,
    pub country: Option<String>,
    pub contract: Option<String>,
    pub cust_nbr: Option<String>,    
    pub gpo: Option<String>,
    pub rebate: f32,
}

// impl From<Row> for PgSale {
//     fn from(row: Row) -> Self {
//         Self {
//             id: row.get("id"),
//             key: row.get("key"),
//             distribution: row.get("distribution"),
//             rep: row.get("rep"),
//             item: row.get("item"),
//             sale: row.get("sale"),
//             quantity: row.get("quantity"),
//             uom: row.get("uom"),
//             date: row.get("date"),
//             customer: row.get("customer"),
//             ship_to_name: row.get("ship_to_name"),
//             addr1: row.get("addr1"),
//             addr2: row.get("addr2"),
//             city: row.get("city"),
//             state: row.get("state"),
//             postal: row.get("postal"),
//             country: row.get("country"),
//             contract: row.get("contract"),
//             cust_nbr: row.get("cust_nbr"),
//             gpo: row.get("gpo"),
//             rebate: row.get("rebate"),
//         }
//     }
// }