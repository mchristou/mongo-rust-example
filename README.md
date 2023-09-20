## mongo-rust-example 

Simple example on how to use MongoDb with Rust.

## Run

Start the application (starts MongoDb instance and executes `cargo run`)
```
./run.sh start
```

Fetch all items
```
curl http://localhost:8080/todo_list
```

Add item
```
curl -X POST http://localhost:8080/todo_list -d '{"title": "Item", "description": "description of item"}' -H "content-type: application/json"
```

Delete item
```
curl -X DELETE http://localhost:8080/todo_list/Item
```

Update item
```
curl -X PUT http://localhost:8080/todo_list/Item -d '{"title": "Item", "description": "Updated item"}' -H "content-type: application/json
```
Stop MongoDb instance
```
./run.sh stop
```
