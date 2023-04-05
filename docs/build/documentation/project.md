Modify the output data using projection template

*Prefix:* `project`

*Value:* Map of fields

*Can start the pipeline:* No

*Steps before:* find, get, sort, limit, offset

*Steps after:* -

## Operators

- Keep - `keep` - keep the respective value

## Usage

In order to keep a field or subfield of the object `keep` operator could be used:

```shell

s|field|:keep,
s|map|:m{
    s|field_2|:keep,
},

```

To set an existing value to a new field `value` notation could be used:

```shell

s|new_field|:value|field|,

```

To set a new value to a field primitives, maps or vectors could be used:

```shell

s|field_1|:s|new value|,

s|field_2|:m{
    s|field|:s|new value|,
},

s|field_3|:v[
    s|new value|,
],

```


## Example

Input:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">test</span>|:<span class="prefix_vector">q</span>[
	<span class="prefix_vector">find</span>[
	],
	<span class="prefix_map">project</span>{
		<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">foo</span>|,
		<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_keep">keep</span>,
		<span class="prefix_string">s</span>|<span class="value_string">vec</span>|:<span class="prefix_vector">v</span>[
			<span class="prefix_keep">keep</span>,
			<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">map</span>|:<span class="prefix_map">m</span>{
			<span class="prefix_string">s</span>|<span class="value_string">bar</span>|:<span class="prefix_keep">keep</span>,
			<span class="prefix_string">s</span>|<span class="value_string">test</span>|:<span class="prefix_string">s</span>|<span class="value_string">test</span>|,
		},
		<span class="prefix_string">s</span>|<span class="value_string">new_field</span>|:<span class="prefix_string">s</span>|<span class="value_string">new_value</span>|,
	},
];
</code></pre>

Output:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_map">objects</span>{
			<span class="prefix_link">test</span>|<span class="value_link">aec71fe0-562d-4ca0-86ec-e3531e7c717f</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">new_field</span>|:<span class="prefix_string">s</span>|<span class="value_string">new_value</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">foo</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">map</span>|:<span class="prefix_map">m</span>{
					<span class="prefix_string">s</span>|<span class="value_string">bar</span>|:<span class="prefix_string">s</span>|<span class="value_string">baz</span>|,
					<span class="prefix_string">s</span>|<span class="value_string">test</span>|:<span class="prefix_string">s</span>|<span class="value_string">test</span>|,
				},
				<span class="prefix_string">s</span>|<span class="value_string">vec</span>|:<span class="prefix_vector">v</span>[
					<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
					<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
				],
			},
			<span class="prefix_link">test</span>|<span class="value_link">369f78b3-bdd3-470e-84d9-b63d9ec2c5e6</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">foo</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">vec</span>|:<span class="prefix_vector">v</span>[
					<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
					<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
				],
				<span class="prefix_string">s</span>|<span class="value_string">new_field</span>|:<span class="prefix_string">s</span>|<span class="value_string">new_value</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">map</span>|:<span class="prefix_map">m</span>{
					<span class="prefix_string">s</span>|<span class="value_string">test</span>|:<span class="prefix_string">s</span>|<span class="value_string">test</span>|,
					<span class="prefix_string">s</span>|<span class="value_string">bar</span>|:<span class="prefix_string">s</span>|<span class="value_string">baz</span>|,
				},
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">fc83bd29-1d18-4286-a6f7-65969937ea26</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">map</span>|:<span class="prefix_map">m</span>{
					<span class="prefix_string">s</span>|<span class="value_string">bar</span>|:<span class="prefix_string">s</span>|<span class="value_string">baz</span>|,
					<span class="prefix_string">s</span>|<span class="value_string">test</span>|:<span class="prefix_string">s</span>|<span class="value_string">test</span>|,
				},
				<span class="prefix_string">s</span>|<span class="value_string">new_field</span>|:<span class="prefix_string">s</span>|<span class="value_string">new_value</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">vec</span>|:<span class="prefix_vector">v</span>[
					<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
					<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
				],
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">0</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">foo</span>|,
			},
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">find_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">3</span>|,
		},
	},
];
</code></pre>