# event-store-db

## Installation(MAC)
```shell
docker pull eventstore/eventstore:23.10.0-alpha-arm64v8

docker run --name esdb-node -it -p 2113:2113 -p 1113:1113 eventstore/eventstore:23.10.0-alpha-arm64v8 --insecure --run-projections=All --enable-atom-pub-over-http
```

## Access 
- [Admin UI](http://localhost:2113)

### Client Sdks 
- [Rust](https://github.com/EventStore/EventStoreDB-Client-Rust)