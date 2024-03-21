# CKC - couchdb-kafka-connector 

Este componente tem como principal objetivo capturar as mudanças que ocorrerem em um banco de dados CouchDB (qualquer
banco de dados criado no CouchDB), via API changes na opção feeds Continuos, ou seja utilizando o recurso streaming, e enviar para um tópico do
Apache Kafka. Para lembrar qual foi o último evento ou mensagem
capturado, em caso de reinicialização do componente,  o último sequêncial (last_seq) é sempre gravado no Apache Zookeeper em um path a ser definido, através
de uma variável de ambiente.
Toda configuração de URL do banco, nome do banco e usuário e senha do banco podem ser feitas através de variáveis de ambiente.
Também as configurações do kafka e do zookeeper podem ser feitas através de variáveis de ambiente.

### Arquitetura da Solução

![Processo principal](./docs/streaming_bcadastro.drawio.svg)

### Tecnologias utilizadas

 - Linguagem Rust v1.76
 - Biblioteca Tokio
 - Biblioteca rdkafka para acessar o Apache Kafka
 - Biblioteca zookeeper para acessar o Apache Zookeeper
 - Docker & Docker compose
 - Kubernetes

### Opções de configurações via variáveis de ambiente

#### Variáveis para configurar o CouchDB

- COUCHDB_HOST : Permite informar a URL do CouchDB. O valor padrão é http://localhost:5984.
- COUCHDB_DATABASE : Permite informar o nome do banco de dados que será capturado as mudanças. O valor padrão é test_db.
- COUCHDB_USERNAME : Permite informar o usuário do banco para autenticação. O valor padrão é admin.
- COUCHDB_PASSWORD : Permite informar a senha do usuário do banco. O valor padrão é senha#123.

Com essas variáveis o CKC consegue conectar ao servidor CouchDB e ao banco de dados que desejamos capturar as mudanças.

#### Variáveis para configurar o acesso ao Kafka

- KAFKA_BROKERS : Lista de brokers do kafka, separados por vírgula. O valor padrão é "localhost:9092,localhost:9093,localhost:9094".
- KAFKA_TOPIC_NAME : Nome do tópico kafka aonde será enviado os eventos ou mensagens capturados pelo CKC a partir do CouchDB. O valor padrão é "kafka_test".
- KAFKA_MESSAGE_TIMEOUT_MS : Tempo em milisegundos, máximo, que o cliente kafka espera para enviar o evento ou mensagem para os brokers. O valor padrão é 5000ms.
- KAFKA_PRODUCER_DELAY_ERROR_MS : Tempo em milisegundos de espera, caso ocorro erro na tentiva de enviar o evento ou mensagem, antes de fazer uma nova tentativa de envio. O valor padrão é 5000ms.

#### Variáveis para configurar o acesso ao Zookeeper

- ZOOKEEPER_SERVERS : Lista de Servidores zookeeper. O valor padrão é "localhost:2181".
- ZOOKEEPER_PATH : Nome do path (caminho) aonde será gravado o último sequencial (last_seq), do evento ou mensagem lido do CouchDB.
                   Não se esqueça que cada banco do CouchDB deve ter um path diferente. O valor padrão é "/bcad_last_seq".

### Para executar localmente (em sua máquina) a aplicação

O primeiro passo e fazer um clone do repositório ou faça download, e na raiz do projeto executar um dos comandos abaixo.

- Observação: É necessário ter o docker e docker-compose instalado no seu computador.

O docker-compose-local.yml vai executar um serviço CouchDB, um cluster Zookeeper (3 serviços), um cluster Kafka (3 serviços) e irá construir e executar o serviço CKC.
Vejamos com executá-lo.

```bash
 docker-compose -f deployments/docker-compose-local.yml up -d --build
```

Para visualizar se os serviços estão executando, digite :

```bash
 docker-compose -f deployments/docker-compose-local.yml ps
```
Para acessar o CouchDB via browse e criar o banco test_db e inserir alguns registros, digite :

```http request
 http://localhost:5984/
```
Para conectar digite o usuário e senha padrão.

Para acessar o container do kafka digite :

```bash
 docker container exec -it kafka1 /bin/bash
```

E para consumir um tópico e visualizar os eventos ou mensagens produzidas pelo CKC no tópico, digite :

```bash
 kafka-console-consumer --bootstrap-server kafka1:19092  --topic kafka_test -from-beginning
```
