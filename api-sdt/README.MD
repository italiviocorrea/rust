# Rust RESTful web API with Actix-web e MongoDB

Este exemplo mostra uma API Restfull CRUD, utilizando como banco de dados o MongoDB.

### Este exemplo utiliza as seguintes tecnologias:
- RUST versão 1.64
- Actix web framework
- MongoDB última versão
- OpenTelemetry com Jaeger

### Para executar a aplicação faça os seguintes passos:

```bash
docker-compose -f deployments/docker-compose-local.yaml up -d --build
```

Após a executar o docker compose, você poderá acessar o swagger-ui, jaeger-ui e também o mongo-express.

Para acessar a swagger-ui digite no seu browser preferido :
```http request
 http://localhost:8080/api/v1/swaggerui/index.html
```

Para acessar a jaeger-ui digite no seu browser preferido :
```http request
 http://localhost:16686/
```

Para acessar a mongo-express digite no seu browser preferido :
```http request
 http://localhost:8081/
```

#### E para finalizar você pode utilizar a aplicação fiber use o comando abaixo
```bash
 docker-compose -f deployments/docker-compose-local.yaml down
```
