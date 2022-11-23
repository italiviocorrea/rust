use std::net::TcpListener;
use actix_web::{App, HttpServer, web};
use api_sdt::app::database::projeto_repository::ProjetoMongoDBRepository;
use api_sdt::app::transport::rest::routes::{general_routes, projeto_routes, swaggerui_routes};
use api_sdt::errors::ApiSdtError;
use api_sdt::settings::Settings;
use api_sdt::state::AppState;
use api_sdt::telemetry::init_telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Le as configurações
    let settings: Settings = Settings::new().unwrap();

    // Inicia a observalidade com OpenTemetry
    init_telemetry(settings.clone());

    // executa o Servidor Rest
    let address = format!("{}:{}", settings.server.host, settings.server.port);
    let listener = TcpListener::bind(address).unwrap();

    let prj_repo = ProjetoMongoDBRepository::init(settings);

    let shared_data = web::Data::new(AppState{
        db: prj_repo.await
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                ApiSdtError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(projeto_routes)
            .configure(swaggerui_routes)
    };

    HttpServer::new(app).listen(listener)?.run().await
}
