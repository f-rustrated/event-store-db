# Installation(MAC)
```shell
docker pull eventstore/eventstore:23.10.0-alpha-arm64v8

docker run --name esdb-node -it -p 2113:2113 -p 1113:1113 eventstore/eventstore:23.10.0-alpha-arm64v8 --insecure --run-projections=All --enable-atom-pub-over-http
```

# Snapshotting 
- EventStoreDB doesn't support snapshotting.

## How to implement snapshotting
1. Events are appended to the stream `post-123`
2. Every nth event, a snapshot is appended to the stream e.g. `post_snapshot-123`
   - use `_` instead of `-` in the suffix `_snapshot`, `-` is used by the system projection 
3. Read backwards a single result from `post_snapshot-123` stream 
    ```java 
    ReadStreamOptions options = ReadStreamOptions.get()
        .backwards()
        .fromEnd();

    ReadResult result = client.readStream("order_snapshot-123", 1 /*maxCount*/, options)
        .get();

    List<ResolvedEvent> events = result.getEvents();
    ```
4. Read forwards from nth revisino from `post-123` stream 
    ```java
    ReadStreamOptions options = ReadStreamOptions.get()
        .forwards()
        .fromRevision(n);

    ReadResult result = client.readStream("order-123", options)
        .get();

    List<ResolvedEvent> events = result.getEvents();
    ```

# Access 
- [Admin UI](http://localhost:2113)

# Client Sdks 
- [Rust](https://github.com/EventStore/EventStoreDB-Client-Rust)

