use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
    contract: String,
    cust_nbr: Option<String>,
    notes: mongodb::bson::Document,
    gpo: Option<String>,
    rebate: f32,
}