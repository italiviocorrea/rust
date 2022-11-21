#![allow(non_snake_case)]
use actix_web::{web,Responder,HttpResponse};
use serde::{Deserialize};
use mongodb::Database;
pub use mongodb::bson::{doc, Document};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use log::info;
use crate::domain::entities::movie_schema::Movie;


#[derive(Deserialize)]
pub struct UserQuery {
    pub name:String,
}

pub async fn getMovie(db:web::Data<Database>, Info: web::Query<UserQuery>) ->impl Responder{

    info!("Mensagem : {:?}", Info.name.clone());

    let  collection = db.collection::<Movie>("Movies");
    let filter = doc!{"name":Info.name.clone()};
    let data = match collection.find_one(filter,None).await {
        Ok(data)=> {
            data
        }
        ,
        Err(err)=> {
            return  HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("X-Hdr", "sample"))
                .status(StatusCode::from_u16(501).unwrap())
                .body(format!("could not post data, reason : {}",err))
        }
    };//end of match statement
    match data {
        Some(data)=>{
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("X-Hdr", "sample"))
                .status(StatusCode::from_u16(200).unwrap())
                .body(format!("Name : {}  \n  Release data : {}  \n ratings : {} ",data.name,data.relese_data,data.rating))
        },
        None=>{
            return  HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("X-Hdr", "sample"))
                .status(StatusCode::from_u16(404).unwrap())
                .body(format!("No movie with name {} exist in the database",Info.name))
        }
    }//end of match statment
}//end of handler