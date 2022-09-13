In this tutorial I'll create a database for the candy store to show the basic AnnaDB features.

## Insert primitive

let's start with categories. Let's represent categories as simple string objects. To do so let's insert the first one into the `categories` collection.

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">categories</span>|:<span class="prefix_vector">insert</span>[
	<span class="prefix_string">s</span>|<span class="value_string">sweets</span>|,
];
</code></pre>

`collection|categories|` shows on which collection the query will be applied. In our case - `categories`

`insert[...]` - is a query step. You can insert one or many objects using the `insert` operation.

`s|sweets|` - is the object to insert. In this case, it is a string primitive. Prefix `s` means that it is a string, `|` wrap the value of the primitive. Other primitive types could be found in the [Data Types section](../../documentation/data_types/).

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_vector">ids</span>[
			<span class="prefix_link">categories</span>|<span class="value_link">bfe9ce56-5d57-4018-8de1-b7933dea26d2</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">insert_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		},
	},
];
</code></pre>

If everything is ok, in result will be returned `ok[...]` vector with responses for all the transaction pipelines. Each response contains `data` and `meta` information. In our case there is only one response with a vector of `ids` in `data` and a number of inserted objects in `meta`

## Insert container

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
		<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_link">categories</span>|<span class="value_link">bfe9ce56-5d57-4018-8de1-b7933dea26d2</span>|,
	},
];
</code></pre>

The query is similar to the previous one, but the object is not a primitive, but a map. The value of the `category` field is a link, that was received after the previous insert.

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_vector">ids</span>[
			<span class="prefix_link">products</span>|<span class="value_link">5412320f-3ad8-46fc-8fcf-716f545a4302</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">insert_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		},
	},
];
</code></pre>

The response is nearly the same as before - link in data and number of inserted objects in meta.

## Get object

Let's retrieve the information about this chocolate bar now. I'll use the `get` operation for this, to the object by id

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">products</span>|:<span class="prefix_vector">get</span>[
	<span class="prefix_link">products</span>|<span class="value_link">5412320f-3ad8-46fc-8fcf-716f545a4302</span>|,
];
</code></pre>

This time I use the `get[...]` query step. Using this step you can retrieve one or many objects using object links.

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_map">objects</span>{
			<span class="prefix_link">products</span>|<span class="value_link">5412320f-3ad8-46fc-8fcf-716f545a4302</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Tony's</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">5.95</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">sweets</span>|,
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
		<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_link">categories</span>|<span class="value_link">bfe9ce56-5d57-4018-8de1-b7933dea26d2</span>|,
	},
];
</code></pre>

I use the same category id for this bar.

## Modify primitive

Let's modify the category to make it more accurate.

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">categories</span>|:<span class="prefix_vector">q</span>[
	<span class="prefix_vector">get</span>[
		<span class="prefix_link">categories</span>|<span class="value_link">bfe9ce56-5d57-4018-8de1-b7933dea26d2</span>|,
	],
	<span class="prefix_vector">update</span>[
		<span class="prefix_map">set</span>{
			<span class="prefix_primitive">root</span>:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
		},
	],
];
</code></pre>

The query here consists of 2 steps. `Get the object by link` step and `modify this object` step. The `update[...]` operation is a vector of update operators. [Read more about the update](../../documentation/update/).

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_vector">ids</span>[
			<span class="prefix_link">categories</span>|<span class="value_link">bfe9ce56-5d57-4018-8de1-b7933dea26d2</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		},
	},
];
</code></pre>

The response of the update operation contains the ids of the updated objects as data and the number of the updated objects as meta.

Let's take a look, at how this affected the chocolate objects.

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">products</span>|:<span class="prefix_vector">find</span>[
];
</code></pre>

To find objects I use the `find[...]` operation. It is a vector of find operators. If it is empty, all the collection objects will be returned.

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_map">objects</span>{
			<span class="prefix_link">products</span>|<span class="value_link">5412320f-3ad8-46fc-8fcf-716f545a4302</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Tony's</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">5.95</span>|,
			},
			<span class="prefix_link">products</span>|<span class="value_link">b507ac61-e251-4e96-b4d1-fe2df9cb4e4f</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Mars</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
			},
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">find_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
		},
	},
];
</code></pre>

The category was changed for both products, as the category object was linked with these objects.

## Modify container

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

The `find` step can stay before the `update` step as well. All the found objects will be updated. Read more about `find` operation and operators [here](../../documentation/find/).

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_vector">ids</span>[
			<span class="prefix_link">products</span>|<span class="value_link">b507ac61-e251-4e96-b4d1-fe2df9cb4e4f</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		},
	},
];
</code></pre>

The response is similar to the previous one.

Here is how all the products look like after update:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_map">objects</span>{
			<span class="prefix_link">products</span>|<span class="value_link">5412320f-3ad8-46fc-8fcf-716f545a4302</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Tony's</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">5.95</span>|,
			},
			<span class="prefix_link">products</span>|<span class="value_link">b507ac61-e251-4e96-b4d1-fe2df9cb4e4f</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Mars</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
			},
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">find_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
		},
	},
];
</code></pre>

## Sort objects

To sort objects I'll use the `sort` operation against the price field

**Request**:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">products</span>|:<span class="prefix_vector">q</span>[
	<span class="prefix_vector">find</span>[
	],
	<span class="prefix_vector">sort</span>[
		<span class="prefix_modifier">asc</span>(<span class="prefix_primitive">value</span>|<span class="value_primitive">price</span>|),
	],
];
</code></pre>

The `sort[...]` operation is a vector of sort operators - `asc` and `desc`. Sort operators are modifiers, that contain paths to the sorting value. The `sort` operation is not an independent step, it can stay only after find-like operations, that return objects. You can read more about sort [here](../../documentation/update/)

**Response**:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_map">objects</span>{
			<span class="prefix_link">products</span>|<span class="value_link">b507ac61-e251-4e96-b4d1-fe2df9cb4e4f</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Mars</span>|,
			},
			<span class="prefix_link">products</span>|<span class="value_link">5412320f-3ad8-46fc-8fcf-716f545a4302</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">category</span>|:<span class="prefix_string">s</span>|<span class="value_string">chocolate</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">price</span>|:<span class="prefix_number">n</span>|<span class="value_number">5.95</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">Tony's</span>|,
			},
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">find_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
		},
	},
];
</code></pre>

Objects in the response are sorted by price now.

It is useful to use `limit` and `offset` operations together with sort. You can read about them in the [documentation](../../documentation/limit/)


## Delete objects

You can use `delete` operation after any find-like step to delete all the found objects. Or it can be used independently to delete the whole collection.

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
			<span class="prefix_link">products</span>|<span class="value_link">5412320f-3ad8-46fc-8fcf-716f545a4302</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		},
	},
];
</code></pre>

The response contains affected ids in `data` and the number of deleted objects in `meta`