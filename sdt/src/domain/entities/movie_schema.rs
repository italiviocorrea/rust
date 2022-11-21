use serde::{Deserialize,Serialize};
use mongodb::options::{IndexOptions};
use mongodb::IndexModel;
use mongodb::Client;
use bson::doc;

#[derive(Deserialize,Serialize)]
pub struct Movie {
    pub name:String,
    pub relese_data:String,
    pub genere:String,
    pub rating:i32
}

pub async fn create_name_unique(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc!{"name":1})
        .options(options)
        .build();
    client
        .database("sdt")  //name of our database
        .collection::<Movie>("Movies") //name of our collection
        .create_index(model, None)
        .await
        .expect("error creating index!");
}