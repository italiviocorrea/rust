use actix_web::web::Json;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, IndexModel};
use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use serde::{Deserialize, Serialize};
use crate::settings::Settings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Versao {
    pub versao: String,
    pub contexto: String,
    pub servicos: Vec<Servico>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Servico {
    pub nome: String,
    pub nome_wsdl: String,
    pub assincrono: bool,
    pub metodos: Vec<Metodo>,
    pub esquemas: Vec<Esquema>,
    pub eventos: Option<Vec<Evento>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evento {
    pub nome: String,
    pub esquema: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Esquema {
    pub nome: String,
    pub descricao: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metodo {
    pub nome: String,
    pub compactar: bool,
    pub envelope_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ambiente {
    pub id: i8,
    pub nome: String,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projeto {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub nome: String,
    pub modelo: i32,
    pub descricao: String,
    pub versoes: Vec<Versao>,
    pub ambientes: Vec<Ambiente>,
}

pub async fn create_projeto_name_unique(client: &Client, settings: Settings) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"nome":1})
        .options(options)
        .build();
    client
        .database(settings.database.name.as_str())  //name of our database
        .collection::<Projeto>("Projetos") //name of our collection
        .create_index(model, None)
        .await
        .expect("erro ao criar indice!");
}

impl From<Json<Projeto>> for Projeto {
    fn from(prj: Json<Projeto>) -> Self {
        Projeto {
            id: None,
            nome: prj.nome.clone(),
            descricao: prj.descricao.clone(),
            versoes: prj.versoes.clone(),
            modelo: prj.modelo.clone(),
            ambientes: prj.ambientes.clone(),
        }
    }
}

