## Server

```shell
docker run --init -p 10001:10001 -t romanright/annadb:0.0.1
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