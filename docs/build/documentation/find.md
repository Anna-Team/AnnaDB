Find objects using comparison and logical operators

*Prefix:* `find`

*Value:* Vector of operators

*Can start the pipeline:* Yes

*Steps before:* find, get, sort, limit, offset

*Steps after:* find, get, sort, limit, offset, update, delete

## Comparison operators

- Equal - `eq{...}`
- Not equal - `neq{...}`
- Greater than - `gt{...}`
- Greater than or equeal - `gte{...}`
- Less than - `lt{...}`
- Less than or equal - `lte{...}`
- Less than or equal - `lte{...}`

## Logical operators

- And - `and[...]`
- Or - `or[...]`
- Not - `not(...)`

In order to compare the value of the object `root path` notation could be used:

```shell
eq{root: s|bar|}
```

In order to compare a field of the object `path to value` notation could be used:

```shell
eq{value|path.to.field|: s|bar|}
```

## Example

Input:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">test</span>|:<span class="prefix_vector">find</span>[
	<span class="prefix_map">gt</span>{
		<span class="prefix_primitive">value</span>|<span class="value_primitive">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
	},
];
</code></pre>

Output:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_map">objects</span>{
			<span class="prefix_link">test</span>|<span class="value_link">6fdafe7d-a87e-433e-a4cf-6ade2fd7a351</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_7</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">7</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">eff6772f-510c-45cd-8c74-0c082260e4dc</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_6</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">6</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">138f78c5-8d81-41e1-bcbe-e5b1e0e92c75</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">8</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_8</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">4e60e33e-5451-41e3-b9c1-e41cb325f0e5</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_5</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">2b7e53d0-f306-4f3a-b814-f0876dc4f3cd</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_9</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">9</span>|,
			},
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">find_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
		},
	},
];
</code></pre>