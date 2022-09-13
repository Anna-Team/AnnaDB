[![shields badge](https://shields.io/badge/-docs-blue)](https://annadb.dev/)
[![docker](https://img.shields.io/docker/v/romanright/annadb)](https://hub.docker.com/repository/docker/romanright/annadb)

![Logo](https://raw.githubusercontent.com/roman-right/AnnaDB/main/docs/build/assets/img/logo_colored.svg?token=GHSAT0AAAAAABXADZHTHTTD4UZR3G6P2J5GYXRFT7Q)

Next-generation developer-first NoSQL database.

AnnaDB moves familiar programming languages' patterns into the databases world to solve the problem of the relations: 

*Every object and sub-object (item of a vector or map) that was stored in AnnaDB has a link id. This link can be placed as a field value of any other object and the database will fetch and process it automatically on all the operations without additional commands in queries.*

Features:

- Flexible object structure
- Relations
- Transactions

## Server

To run AnnaDB locally please type the next command in the terminal:

```shell
docker run --init -p 10001:10001 -t romanright/annadb:0.1.0
```

## Client

AnnaDB shell client is an interactive terminal application that connects to the DB instance, validates and handles queries. It fits well to play with query language or work with the data manually.

![AnnaDB shell](https://raw.githubusercontent.com/roman-right/AnnaDB/0c9f00f53f21184fe166c5c70d417f0ed4bcf01b/docs/build/assets/img/shell.png)

It can be installed via `pip`

```shell
pip install annadb
```

Run

```shell
annadb --uri annadb://localhost:10001
```

## Links

- [Documentation](https://annadb.dev)
- [Docker Hub](https://hub.docker.com/repository/docker/romanright/annadb)