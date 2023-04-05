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
			<span class="prefix_link">test</span>|<span class="value_link">45f3bdbf-909c-4cd9-bb2b-09745b2d4dce</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_8</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">8</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">43057373-8661-46c5-ae48-dd41cf0d58a5</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_9</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">9</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">01ca2c96-cdbe-4794-a222-3e4b5dc691a4</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">7</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_7</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">f9d12ac9-acd7-4d53-97fc-5f936ab5b463</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_6</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">6</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">1b7838a5-0d85-454e-92c4-856079eeec4f</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_5</span>|,
			},
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">find_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
		},
	},
];
</code></pre>