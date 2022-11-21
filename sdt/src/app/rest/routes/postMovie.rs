#![allow(non_snake_case)]
use actix_web::{web,Responder,HttpResponse};
use serde::{Deserialize};
use mongodb::Database;
pub use mongodb::bson::{doc, Document};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use crate::domain::entities::movie_schema::Movie;
use log::info;

#[derive(Deserialize)]
pub struct UserData {
    pub name:String,
    pub relese_data:String,
    pub genere:String,
    pub rating:i32
}
pub async fn postMovie(db:web::Data<Database>, Info: web::Json<UserData>) ->impl Responder{
    let data = Movie{
        name:Info.name.clone(),
        relese_data:Info.relese_data.clone(),
        genere:Info.genere.clone(),
        rating:Info.rating.clone()
    };
    info!("Mensagem : {:?}", data.rating);
    let  collection = db.collection::<Movie>("Movies");
    match collection.insert_one(data,None).await {
        Ok(_ok)=> {
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("X-Hdr", "sample"))
                .status(StatusCode::from_u16(200).unwrap())
                .body(format!("data posted"))
        },
        Err(err)=> {
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("X-Hdr", "sample"))
                .status(StatusCode::from_u16(501).unwrap())
                .body(format!("could not post data, reason : {}",err))
        }
    } // end of match statement
} // end of handler