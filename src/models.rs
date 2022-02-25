//use uuid::Uuid;
//use bytes::{Bytes};
use crate::schema::*;
use serde::{Serialize, Deserialize};
use diesel::Queryable;

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Document {
    pub id: i32,
    pub name: String,
    //pub content: Bytes,
}

#[derive(Deserialize, Insertable)]
#[table_name = "documents"]
pub struct NewDocument {
    pub name: String,
}

//#[table_name = "cmolsDocuments"]