use actix_web::{HttpResponse, Responder, web};
use tracing::instrument;

use crate::domain::models::projeto::Projeto;
use crate::errors::ApiSdtError;
use crate::state::AppState;

#[instrument()]
pub async fn health_check_handler() -> Result<HttpResponse, ApiSdtError> {
    Ok(HttpResponse::Ok().json("API SDT esta em funcionamento!"))
}

#[instrument()]
pub async fn projeto_create(
    new_projeto: web::Json<Projeto>,
    app_state: web::Data<AppState>, ) -> impl Responder {
    println!("Criando o projeto {} no BD", new_projeto.nome);

    // Faz uma cópia do projeto
    let mut data = Projeto {
        id: None,
        nome: new_projeto.nome.clone(),
        modelo: new_projeto.modelo.clone(),
        descricao: new_projeto.descricao.clone(),
        versoes: new_projeto.versoes.clone(),
        ambientes: new_projeto.ambientes.clone(),
    };

    let action = app_state.db.create(&new_projeto).await;

    match action {
        Ok(_result) => {
            // Insere o ID gerado pelo BD
            data.id = _result.inserted_id.as_object_id();
            // Gera a resposta com o projeto inserido
            HttpResponse::Ok().json(data)
        },
        Err(_err) => HttpResponse::InternalServerError().json("Não foi possível inserir o projeto no BD"),
    }
}

#[instrument()]
pub async fn projeto_update(
    nome: web::Path<String>,
    projeto: web::Json<Projeto>,
    app_state: web::Data<AppState>, ) -> impl Responder {
    let result = app_state.db.update(nome.into_inner(),&projeto).await;
    match result {
        Ok(_result) => HttpResponse::Ok().json(&projeto),
        Err(_err) => HttpResponse::InternalServerError().json("Não foi possível atualizar o projeto no BD"),
    }
}
#[instrument()]
pub async fn find_by_name_projeto(
    app_state: web::Data<AppState>,
    nome: web::Path<String>) -> Result<HttpResponse, ApiSdtError> {
    println!("Pesquisando o projeto no BD");

    app_state.db.find_by_name(nome.into_inner())
        .await
        .map(|projeto| HttpResponse::Ok().json(projeto))
}

#[instrument()]
pub async fn delete_projeto(
    app_state: web::Data<AppState>,
    nome: web::Path<String>) -> Result<HttpResponse, ApiSdtError> {
    println!("Excluíndo o projeto do BD");
    app_state.db.delete(nome.into_inner())
        .await
        .map(|_rows_deleted| HttpResponse::Ok().json("Projeto removido com sucesso!"))
}

#[instrument()]
pub async fn findall_projeto(
    app_state: web::Data<AppState>) -> Result<HttpResponse, ApiSdtError> {
    println!("Pesquisando todos os projetos do BD");

    app_state.db.find_all()
        .await
        .map(|projetos| HttpResponse::Ok().json(projetos))
}
