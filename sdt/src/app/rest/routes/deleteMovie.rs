#![allow(non_snake_case)]
use actix_web::{web,Responder,HttpResponse};
use serde::{Deserialize};
use mongodb::Database;
pub use mongodb::bson::{doc, Document};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use crate::domain::entities::movie_schema::Movie;


#[derive(Deserialize)]
pub struct UserQuery {
    pub name:String,
}

pub async fn deleteMovie(db:web::Data<Database>, Info: web::Query<UserQuery>) ->impl Responder{
    let  collection = db.collection::<Movie>("Movies");
    let filter = doc!{"name":Info.name.clone()};
    let _data = match collection.delete_one(filter, None).await {
        Ok(_data)=> {
            return  HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("X-Hdr", "sample"))
                .status(StatusCode::from_u16(201).unwrap())
                .body(format!("movie deleted"))
        },
        Err(err)=> {
            return  HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("X-Hdr", "sample"))
                .status(StatusCode::from_u16(501).unwrap())
                .body(format!("internal server error : {}",err))
        }
    };//end of match statement
}//end of handler