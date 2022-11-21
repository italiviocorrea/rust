use actix_web:: {web, App, HttpServer};
use mongodb::{Database};
use std::net::TcpListener;
use actix_web::dev:: Server;
use crate::deleteMovie::deleteMovie;
use crate::getMovie::getMovie;
use crate::postMovie::postMovie;
use crate::updateMovie::updateMovie;

pub fn run(listener:TcpListener,db:Database)->Result<Server, std::io:: Error> {
    let db = web::Data::new(db);
    let _server = HttpServer::new(move|| {
        App:: new()
            .route("/postMovie", web::post().to(postMovie))
            .route("/updateMovie", web::get().to(updateMovie))
            //you could use patch method also
            .route("/deleteMovie", web::delete().to(deleteMovie))
            .route("/showMovie", web::get().to(getMovie))
            .app_data(db.clone())
    }).listen(listener).expect(" ")
        .run();
    Ok(_server)
}