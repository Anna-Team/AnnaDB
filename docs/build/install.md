## Server

```shell
docker run --init -p 10001:10001 -t romanright/annadb:0.1.0
```

Alternatively, you can persist your data:

```shell
docker run --init -p 10001:10001 -t -v "$(pwd)/data:/app/warehouse" romanright/annadb:0.1.0
```

## Client shell

Install

```shell
pip install annadb
```

Run

```shell
annadb --uri annadb://localhost:10001
```

## Playground

Alternatively, you can try AnnaDB using the public playground connection string:

```shell
annadb --uri annadb://playground.annadb.dev:10001
```
