Insert one or many primitive or container objects

*Prefix:* `insert`

*Value:* Vector of object

*Can start the pipeline:* Yes

*Steps before:* -

*Steps after:* -


## Example

Input:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">test</span>|:<span class="prefix_vector">insert</span>[
	<span class="prefix_string">s</span>|<span class="value_string">foo</span>|,
	<span class="prefix_number">n</span>|<span class="value_number">100</span>|,
	<span class="prefix_bool">b</span>|<span class="value_bool">true</span>|,
	<span class="prefix_vector">v</span>[
		<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
		<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
		<span class="prefix_number">n</span>|<span class="value_number">3</span>|,
	],
	<span class="prefix_map">m</span>{
		<span class="prefix_string">s</span>|<span class="value_string">bar</span>|:<span class="prefix_string">s</span>|<span class="value_string">baz</span>|,
	},
];
</code></pre>

Output:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_vector">ids</span>[
			<span class="prefix_link">test</span>|<span class="value_link">196f09d5-8c60-4016-8b5c-37a36b386e5d</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">97a94e8a-c6b0-4b9e-ba01-7d9174d9d23c</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">77dee826-c1ec-4435-b03b-7ec4dcc07285</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">02368358-7493-4715-a1a3-59e43861a412</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">af609b1d-740a-4bdb-9838-22befad9038b</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">insert_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
		},
	},
];
</code></pre>