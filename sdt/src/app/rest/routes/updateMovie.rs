#![allow(non_snake_case)]
use actix_web::{web,Responder,HttpResponse};
use serde::{Deserialize};
use mongodb::Database;
pub use mongodb::bson::{doc, Document};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use crate::domain::entities::movie_schema::Movie;


#[derive(Deserialize)]
pub struct UserData {
    pub name:String,
    pub relese_date:Option<String>,
    pub genere:Option<String>,
    pub rating:Option<i32>
}

pub async fn updateMovie(db:web::Data<Database>, Info: web::Json<UserData>) ->impl Responder{
    let  collection = db.collection::<Movie>("Movies");
    let Movie = collection.find_one(doc!{"name":Info.name.clone()},None).await;
    let Movie  = match Movie {
        Ok(movie)=>movie,
        Err(_err)=>  {
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("X-Hdr", "sample"))
                .status(StatusCode::from_u16(501).unwrap())
                .body(format!("internal server error"))
        }
    };//end of match
    let Movie = match Movie{
        Some(Movie)=> Movie,
        None=>{
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("X-Hdr", "sample"))
                .status(StatusCode::from_u16(401).unwrap())
                .body(format!("no movie with name {} exist",Info.name))
        }
    };//end of match statement
    let releasedate = if Info.relese_date == None{ Movie.relese_data} else { Info.relese_date.clone().unwrap()};
    let genere = if Info.genere == None{ Movie.genere} else { Info.genere.clone().unwrap()};
    let rating = if Info.rating == None{ Movie.rating} else { Info.rating.unwrap()};
    let filter = doc!{"name":Info.name.clone()};
    let update = doc!{"$set":{"relese_data":releasedate,"genere":genere,"rating":rating}};
    match collection.update_one(filter,update,None).await{
        Err(_err)=>{
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("X-Hdr", "sample"))
                .status(StatusCode::from_u16(501).unwrap())
                .body(format!("internal server error {} ",_err))
        },
        Ok(_ok)=>{
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .insert_header(("X-Hdr", "sample"))
                .status(StatusCode::from_u16(201).unwrap())
                .body(format!("update finished"))
        }
    }//end of match arm
}//end of handler