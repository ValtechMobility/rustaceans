# create a project

cargo new <project-name>

# project bauen

rustc <filename>

cargo build im project-root

cargo test

# project laufen lassen

cargo run

# rust projects

## NATS

### start a nats server

```docker run -p 4222:4222 -ti nats:latest```

### simple config fur authentication

add nats-server.conf with content

```
server_name = "mynats"
listen: 127.0.0.1:4222
authorization {
  users = [
    { user = "admin", password = "geheim" }
  ]
}
```

and start with


```docker run -p 4222:4222 -v $(pwd)/nats-server.conf:/etc/nats/nats-server.conf -ti nats:latest -c /etc/nats/nats-server.conf```

### add persistierung and co

add nats-server.conf with content

```
jetstream {
  store_dir = "/data/jetstream"  # Wo persistente Daten gespeichert werden
  max_mem_store = 128Mb
  max_file_store = 512Mb
}
```

and start with


```docker run -p 4222:4222 -v $(pwd)/jetstream:/data/jetstream -v $(pwd)/nats-server.conf:/etc/nats/nats-server.conf -ti nats:latest -js -c /etc/nats/nats-server.conf```

### playin with docker / network

docker network rm nats-net

docker remove nats-server

### starten

erst ```./server``` für NATS samt netzwerk, dann ```./run -nt``` und in der cmd dann ```cargo run```

nur tests laufen lassen ohne NATS, etc. ```./run -r```

### nats in docker

```docker run -it --rm synadia/nats-box```

nats pub -s nats://myuser:mypassword@34.78.28.109:30222 test "hello von Stephan"

curl nats://34.78.28.109:30222

### dtos in rust


 