use crate::app::database::projeto_repository::ProjetoMongoDBRepository;

#[derive(Debug)]
pub struct AppState {
    pub db: ProjetoMongoDBRepository,
}

