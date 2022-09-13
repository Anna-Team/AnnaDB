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
			<span class="prefix_link">test</span>|<span class="value_link">24362508-862c-40b5-a628-290c2d54a421</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">9</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_9</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">13ba0422-1f79-4b8b-ac5c-1240134ca05e</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_7</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">7</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">3f0a21ad-c778-46a7-a72a-00c001b42e95</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">8</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_8</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">7122d418-0453-4a5e-b2b6-02cf5776b6db</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_5</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">fe391a96-aaff-492f-9e09-20df5464f4ba</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">6</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_6</span>|,
			},
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">find_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
		},
	},
];
</code></pre>