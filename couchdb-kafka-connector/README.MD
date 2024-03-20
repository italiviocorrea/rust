docker container exec -it kafka1 /bin/bash
kafka-console-consumer --bootstrap-server kafka1:19092  --topic kafka_test -from-beginning


para restartar docker automaticamente
docker run --rm --restart always 
       --env COUCHDB_HOST="http://127.0.0.1:5984" 
       --env COUCHDB_DATABASE="test_db" 
       --env COUCHDB_USERNAME="admin" 
       --env COUCHDB_PASSWORD="senha#123" 
       ctk:local 
