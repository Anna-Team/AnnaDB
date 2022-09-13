[![shields badge](https://shields.io/badge/-docs-blue)](https://annadb.dev/)
[![docker](https://img.shields.io/docker/v/romanright/annadb)](https://hub.docker.com/repository/docker/romanright/annadb)

![Logo](https://raw.githubusercontent.com/roman-right/AnnaDB/main/docs/build/assets/img/logo_colored.svg?token=GHSAT0AAAAAABXADZHTHTTD4UZR3G6P2J5GYXRFT7Q)

Next-generation developer-first NoSQL database.

AnnaDB is an in-memory data store with disk persistence. It is written with Rust, a memory-safe compilable language. AnnaDB is fast and safe enough to be and the main data storage, and the cache layer.

**Features**

- Flexible object structure - simple primitives and complicated nested containers could be stored in AnnaDB
- Relations - you can link any object to another, and AnnaDB will resolve this relation on finds, updates, and other operations.
- Transactions - out of the box

## Collections

AnnaDB stores objects in collections. Collections are analogous to tables in SQL databases. 

Every object and sub-object (item of a vector or map) that was stored in AnnaDB has a link (id). This link consists of the collection name and unique uuid4 value. One object can contain links to objects from any collections - AnnaDB will fetch and process them on all the operations automatically without additional commands (joins or lookups)

## TySON

The AnnaDB query language uses the `TySON` format. The main difference from other data formats is that each item has a value and prefix. The prefix can mark the data or query type (as it is used in AnnaDB) or any other useful for the parser information. This adds more flexibility to the data structure design - it is allowed to use as many custom data types as the developer needs.

You can read more about the `TySON` format [here](https://github.com/roman-right/tyson)

## Data Types

There are primitive and container data types in AnnaDB.

Primitive data types are a set of basic types whose values can not be decoupled. In TySON, primitives are represented as `prefix|value|` or `prefix` only. Prefix in AnnaDB shows the data type. For example, the string `test` will be represented as `s|test|`, where `s` - is a prefix that marks data as a string, and `test` is the actual value.

Container data types keep primitive and container objects using specific rules. There are only two container types in AnnaDB for now. Maps and vectors.

- Vectors are ordered sets of elements of any type. Example: `v[n|1|,n|2|,n|3|,]`
- Maps are associative arrays. Example: `m{ s|bar|: s|baz|,}`

More information about AnnaDB data types can be found in the [documentation](https://annadb.dev/documentation/data_types/)

## Query

Query in AnnaDB is a pipeline of steps that should be applied in the order it was declared. The steps are wrapped into a vector with the prefix `q` - query.

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">test</span>|:<span class="prefix_vector">q</span>[
   <span class="prefix_vector">find</span>[
   ],
   <span class="prefix_vector">sort</span>[
      <span class="prefix_modifier">asc</span>(<span class="prefix_primitive">value</span>|<span class="value_primitive">num</span>|),
   ],
   <span class="prefix_modifier">limit</span>(<span class="prefix_number">n</span>|<span class="value_number">5</span>|),
];
</code></pre>

If the pipeline has only one step, the `q` vector is not needed.

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">test</span>|:<span class="prefix_vector">find</span>[
   <span class="prefix_map">gt</span>{
      <span class="prefix_primitive">value</span>|<span class="value_primitive">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
   },
];
</code></pre>

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