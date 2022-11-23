extern crate futures;

use mongodb::error::Error;
use futures::stream::TryStreamExt;
use mongodb::{Client, Collection};
use mongodb::bson::{doc};
use mongodb::results::{InsertOneResult, UpdateResult};

use crate::domain::models::projeto::Projeto;
use crate::errors::ApiSdtError;
use crate::settings::Settings;

#[derive(Debug)]
pub struct ProjetoMongoDBRepository {
    col: Collection<Projeto>,
}

impl ProjetoMongoDBRepository {
    pub async fn init(settings: Settings) -> Self {
        let url = format!("mongodb://{}:{}@{}:{}",
                          settings.database.username,
                          settings.database.password,
                          settings.database.host,
                          settings.database.port);

        let client = Client::with_uri_str(url)
            .await
            .expect("Erro ao conectar com o banco de dados!");
        let db = client.database(&settings.database.name);
        let col: Collection<Projeto> = db.collection("Projetos");
        //create_projeto_name_unique(&client, settings).await;

        ProjetoMongoDBRepository { col }
    }


    pub async fn find_by_name(&self, name: String) -> Result<Projeto, ApiSdtError> {
        let filter = doc! {"nome": name};
        return match self.col.find_one(filter, None).await {
            Ok(Some(data)) => Ok(data),
            Ok(None) => Err(ApiSdtError::NotFound("Não encontrado projeto com este nome no BD!".into())),
            Err(_err) => Err(ApiSdtError::DBError("Erro ao pesquisar no BD!".into())),
        };
    }

    pub async fn find_all(&self) -> Result<Vec<Projeto>, ApiSdtError> {
        let mut cursors = self.col.find(None, None).await?;

        let mut data: Vec<Projeto> = Vec::new();
        while let Some(result) = cursors.try_next().await? {
            data.push(result);
        }
        return match data.len() {
            0 => Err(ApiSdtError::NotFound("Não foi encontrado projetos no BD!".into())),
            _ => Ok(data),
        };
    }

    pub async fn create(&self, new_projeto: &Projeto) -> Result<InsertOneResult, Error> {
        // Insere o projeto no banco de dados
        self.col.insert_one(new_projeto, None).await
    }

    pub async fn update(&self, name: String, update_projeto: &Projeto) -> Result<UpdateResult, Error> {
        let filter = doc! {"nome":&name};
        self.col.replace_one(filter, update_projeto, None).await
    }


    pub async fn delete(&self, name: String) -> Result<u64, ApiSdtError> {
        let filter = doc! {"nome":name.clone()};
        let result = self.col.delete_one(filter, None).await?;
        return match result.deleted_count {
            1 => Ok(result.deleted_count),
            0 => Err(ApiSdtError::NotFound("Não foi possível encontrar o projeto, para excluir no BD!".into())),
            _ => Err(ApiSdtError::DBError("Erro ao tentar excluir o projeto no BD!".into())),
        };
    }
}

