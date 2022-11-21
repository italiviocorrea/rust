use bson::doc;
use bson::oid::ObjectId;
use mongodb::{Client, IndexModel};
use mongodb::options::IndexOptions;
use serde::{Deserialize, Serialize};

use crate::Movie;

#[derive(Deserialize, Serialize)]
pub struct Versao {
    pub versao: String,
    pub contexto: String,
}

#[derive(Deserialize, Serialize)]
pub struct Servico {
    pub nome: String,
    pub nome_wsdl: String,
    pub assincrono: bool,
    pub metodos: Vec<Metodo>,
    pub esquemas: Vec<Esquema>,
    pub eventos: Vec<Evento>,
}

#[derive(Deserialize, Serialize)]
pub struct Evento {
    pub nome: String,
    pub esquema: String,
}

#[derive(Deserialize, Serialize)]
pub struct Esquema {
    pub nome: String,
    pub descricao: String,
}

#[derive(Deserialize, Serialize)]
pub struct Metodo {
    pub nome: String,
    pub compactar: bool,
    pub envelope_template: String,
}

#[derive(Deserialize, Serialize)]
pub struct Ambiente {
    pub id: i8,
    pub nome: String,
    pub uri: String,
}

#[derive(Deserialize, Serialize)]
pub struct Projeto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub nome: String,
    pub modelo: i8,
    pub descricao: String,
    pub versoes: Vec<Versao>,
    pub ambientes: Vec<Ambiente>,
}

pub async fn create_projeto_name_unique(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"nome":1})
        .options(options)
        .build();
    client
        .database("sdt_db")  //name of our database
        .collection::<Projeto>("Projetos") //name of our collection
        .create_index(model, None)
        .await
        .expect("erro ao criar indice!");
}
