I'm excited to introduce [AnnaDB](https://annadb.dev) - the next-generation developer-first NoSQL data store.

I work with many small projects daily - proofs of concepts and experiments with new frameworks or patterns. For these purposes, I needed a database that works with flexible data structures, as I change it frequently during my experiments. And it must support relations out of the box, as this is a natural part of the structures' design - links to other objects. I tried a lot (if not all) databases, but nothing fit my requirements well. So, I decided to make my own then. This is how AnnaDB was born.

AnnaDB is an in-memory data store with disk persistence. It is written with Rust, a memory-safe compilable language. AnnaDB is fast and safe enough to be and the main data storage, and the cache layer.

**Features**

- Flexible object structure - simple primitives and complicated nested containers could be stored in AnnaDB
- Relations - you can link any object to another, and AnnaDB will resolve this relation on finds, updates, and other operations.
- Transactions - out of the box

## Basics

I want to start with the basic concepts and examples of the syntax here and continue with the usage example.

### Collections

AnnaDB stores objects in collections. Collections are analogous to tables in SQL databases. 

Every object and sub-object (item of a vector or map) that was stored in AnnaDB has a link (id). This link consists of the collection name and unique uuid4 value. One object can contain links to objects from any collections - AnnaDB will fetch and process them on all the operations automatically without additional commands (joins or lookups)

### TySON

The AnnaDB query language uses the `TySON` format. The main difference from other data formats is that each item has a value and prefix. The prefix can mark the data or query type (as it is used in AnnaDB) or any other useful for the parser information. This adds more flexibility to the data structure design - it is allowed to use as many custom data types as the developer needs.

You can read more about the `TySON` format [here](https://github.com/roman-right/tyson)

### Data Types

There are primitive and container data types in AnnaDB.

Primitive data types are a set of basic types whose values can not be decoupled. In TySON, primitives are represented as `prefix|value|` or `prefix` only. Prefix in AnnaDB shows the data type. For example, the string `test` will be represented as `s|test|`, where `s` - is a prefix that marks data as a string, and `test` is the actual value.

Container data types keep primitive and container objects using specific rules. There are only two container types in AnnaDB for now. Maps and vectors.

- Vectors are ordered sets of elements of any type. Example: `v[n|1|,n|2|,n|3|,]`
- Maps are associative arrays. Example: `m{ s|bar|: s|baz|,}`

More information about AnnaDB data types can be found in the [documentation](https://annadb.dev/documentation/data_types/)

### Query

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

### Server

To run AnnaDB locally please type the next command in the terminal:

```shell
docker run --init -p 10001:10001 -t romanright/annadb:0.1.0
```

### Client

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

## Usage example

You are prepared for the fun part of the article now. Let's play with AnnaDB!

I'll create a database for the candy store to show the features.

### Insert primitive

Let's start with categories. I'll represent categories as simple string objects. Let's insert the first one into the `categories` collection.

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">categories</span>|:<span class="prefix_vector">insert</span>[
	<span class="prefix_string">s</span>|<span class="value_string">sweets</span>|,
];
</code></pre>

`collection|categories|` shows on which collection the query will be applied. In our case - `categories`.

`insert[...]` - is a query step. You can insert one or many objects using the `insert` operation.

`s|sweets|` - is the object to insert. In this case, it is a string primitive. Prefix `s` means that it is a string, `|` wrap the value of the primitive. Other primitive types could be found in the [Data Types section](https://annadb.dev/documentation/data_types/).

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_vector">ids</span>[
			<span class="prefix_link">categories</span>|<span class="value_link">3346ea3e-a531-4d68-8ace-07efcb557946</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">insert_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		},
	},
];
</code></pre>

If everything is ok, the result will have an `ok[...]` vector with responses for all the transaction pipelines. Each response contains `data` and `meta` information. In our case, there is only one response with a vector of `ids` in `data` and a number of inserted objects in `meta`.

### Insert container

Let's insert a more complicated object now - a chocolate bar. It will have fields:

- name
- price
- category

For the category, I'll use the already created one.

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">products</span>|:<span class="prefix_vector">insert</span>[
	<span class="prefix_map">m</span>{
		<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Tony's</span>|,
		<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">5.95</span>|,
		<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_link">categories</span>|<span class="value_link">3346ea3e-a531-4d68-8ace-07efcb557946</span>|,
	},
];
</code></pre>

The query is similar to the previous one, but the object is not a primitive but a map. The value of the `category` field is a link that was received after the previous insert.

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_vector">ids</span>[
			<span class="prefix_link">products</span>|<span class="value_link">ff2a7d6c-c46a-4be6-9195-25acce5d13c5</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">insert_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		},
	},
];
</code></pre>

The response is nearly the same as before - link in data and number of inserted objects in meta.

### Get object

Let's retrieve the information about this chocolate bar now. I'll use the `get` operation for this, to the object by id

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">products</span>|:<span class="prefix_vector">get</span>[
	<span class="prefix_link">products</span>|<span class="value_link">ff2a7d6c-c46a-4be6-9195-25acce5d13c5</span>|,
];
</code></pre>

This time I use the `get[...]` query step. Using this step you can retrieve one or many objects using object links.

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_map">objects</span>{
			<span class="prefix_link">products</span>|<span class="value_link">ff2a7d6c-c46a-4be6-9195-25acce5d13c5</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">5.95</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">sweets</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Tony's</span>|,
			},
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">get_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		},
	},
];
</code></pre>

In the response here you can see the `objects{...}` map, where keys are links to objects and values are objects. `objects{}` map keeps the order - it will return objects in the same order as they were requested in the get step, or as they were sorted by the sort step.

The category was fetched automatically and the value was returned.

Let's insert another chocolate bar there to have more objects in the collection:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">products</span>|:<span class="prefix_vector">insert</span>[
	<span class="prefix_map">m</span>{
		<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Mars</span>|,
		<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
		<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_link">categories</span>|<span class="value_link">3346ea3e-a531-4d68-8ace-07efcb557946</span>|,
	},
];
</code></pre>

I use the same category id for this bar.

### Modify primitive

Let's modify the category to make it more accurate.

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">categories</span>|:<span class="prefix_vector">q</span>[
	<span class="prefix_vector">get</span>[
		<span class="prefix_link">categories</span>|<span class="value_link">3346ea3e-a531-4d68-8ace-07efcb557946</span>|,
	],
	<span class="prefix_vector">update</span>[
		<span class="prefix_map">set</span>{
			<span class="prefix_primitive">root</span>:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
		},
	],
];
</code></pre>

The query here consists of 2 steps. `Get the object by link` step and `modify this object` step. The `update[...]` operation is a vector of update operators. [Read more about the update](https://annadb.dev/documentation/update/).

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_vector">ids</span>[
			<span class="prefix_link">categories</span>|<span class="value_link">3346ea3e-a531-4d68-8ace-07efcb557946</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		},
	},
];
</code></pre>

The response of the update operation contains the ids of the updated objects as data and the number of the updated objects as meta.

Let's take a look at how this affected the chocolate objects.

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">products</span>|:<span class="prefix_vector">find</span>[
];
</code></pre>

To find objects, I use the `find[...]` operation. It is a vector of find operators. If it is empty, all the collection objects will be returned.

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_map">objects</span>{
			<span class="prefix_link">products</span>|<span class="value_link">ff2a7d6c-c46a-4be6-9195-25acce5d13c5</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Tony's</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">5.95</span>|,
			},
			<span class="prefix_link">products</span>|<span class="value_link">03fc1604-7d7f-4fb6-bf82-6273ed3e4320</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Mars</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
			},
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">find_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
		},
	},
];
</code></pre>

The category was changed for both products, as the category object was linked with these objects.

### Modify container

Now I'll increase the price of the bars, where it is less than 2

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">products</span>|:<span class="prefix_vector">q</span>[
	<span class="prefix_vector">find</span>[
		<span class="prefix_map">lt</span>{
			<span class="prefix_primitive">value</span>|<span class="value_primitive">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">3</span>|,
		},
	],
	<span class="prefix_vector">update</span>[
		<span class="prefix_map">inc</span>{
			<span class="prefix_primitive">value</span>|<span class="value_primitive">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
		},
	],
];
</code></pre>

The `find` step can stay before the `update` step as well. All the found objects will be updated. Read more about `find` operation and operators [here](https://annadb.dev/documentation/find/).

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_vector">ids</span>[
			<span class="prefix_link">products</span>|<span class="value_link">03fc1604-7d7f-4fb6-bf82-6273ed3e4320</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		},
	},
];
</code></pre>

The response is similar to the previous one.

Here is how all the products look like after the update:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_map">objects</span>{
			<span class="prefix_link">products</span>|<span class="value_link">ff2a7d6c-c46a-4be6-9195-25acce5d13c5</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">5.95</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Tony's</span>|,
			},
			<span class="prefix_link">products</span>|<span class="value_link">03fc1604-7d7f-4fb6-bf82-6273ed3e4320</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Mars</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
			},
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">find_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
		},
	},
];
</code></pre>

### Sort objects

To sort objects, I'll use the `sort` operation against the price field.

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">products</span>|:<span class="prefix_vector">q</span>[
	<span class="prefix_vector">find</span>[
	],
	<span class="prefix_vector">sort</span>[
		<span class="prefix_modifier">asc</span>(<span class="prefix_primitive">value</span>|<span class="value_primitive">price</span>|),
	],
];
</code></pre>

The `sort[...]` operation is a vector of sort operators - `asc` and `desc`. Sort operators are modifiers that contain paths to the sorting value. The `sort` operation is not an independent step, it can stay only after find-like operations that return objects. You can read more about sort [here](https://annadb.dev/documentation/update/)

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_map">objects</span>{
			<span class="prefix_link">products</span>|<span class="value_link">03fc1604-7d7f-4fb6-bf82-6273ed3e4320</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Mars</span>|,
			},
			<span class="prefix_link">products</span>|<span class="value_link">ff2a7d6c-c46a-4be6-9195-25acce5d13c5</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Tony's</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">5.95</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
			},
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">find_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
		},
	},
];
</code></pre>

Objects in the response are sorted by price now.

It is useful to use `limit` and `offset` operations together with sort. You can read about them in the [documentation](https://annadb.dev/documentation/limit/)

### Delete objects

After any find-like step, you can use the `delete` operation to delete all the found objects. Or it can be used independently to delete the whole collection.

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">products</span>|:<span class="prefix_vector">q</span>[
	<span class="prefix_vector">find</span>[
		<span class="prefix_map">gt</span>{
			<span class="prefix_primitive">value</span>|<span class="value_primitive">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
		},
	],
	<span class="prefix_primitive">delete</span>,
];
</code></pre>

The `delete` operation is a primitive without value.

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_vector">ids</span>[
			<span class="prefix_link">products</span>|<span class="value_link">ff2a7d6c-c46a-4be6-9195-25acce5d13c5</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		},
	},
];
</code></pre>

The response contains affected ids in `data` and the number of deleted objects in `meta`.

## Using from your app

AnnaDB has a Python driver. It has an internal query builder - you don't need to learn AnnaDB query syntax to work with it. But it supports raw querying too.

- [Link to the PyPI repo](https://pypi.org/project/annadb/)
- [Python tutorial](https://annadb.dev/tutorial/python/)

I'll add drivers for other languages soon. If you can help me with it, I'll be more than happy :)

## Plans

This is the very early version of the database. It can already do things, and I use it in a few of my projects. But there are many features to work on yet.

### Drivers

I plan to add drivers to support the most popular languages, like `JS`, `Rust`, `Go`, and others. If you can help with this - please get in touch with me.

### Rights management

This is probably the most important feature to implement. Authentication, authorizations, roles, etc.

### Performance increase

There are many performance-related things to improve now. 

### Query features

- Projections
- More find and update operators
- Developer experience improves

### Data Types

I plan to add more data types like geo points and graph vertices to make AnnaDB more comfortable working with different data fields.

### Managed service

My big goal is to make a managed data store service. Hey, AWS, Google Cloud, MS Azure, I'm ready for collaborations! ;)

## Links

- Documentation - <https://annadb.dev>
- GitHub Repo - <https://github.com/roman-right/AnnaDB>
- Python Driver - <https://pypi.org/project/annadb/>
- My Twitter - <https://twitter.com/roman_the_right>

*If you face any bug or weird behavior, please, let me know.*