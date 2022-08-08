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
	<span class="prefix_bool">b</span>|<span class="value_bool">True</span>|,
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
			<span class="prefix_link">test</span>|<span class="value_link">daaa8900-8edd-4165-949a-de06646856af</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">51dc6d98-8e8a-4478-831e-5d292a556733</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">b84c788d-2eb9-4e8e-999d-5cb2fb1a4fdc</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">71612f10-7921-4afe-a9de-6175f963a6ad</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">96ebe097-142d-404c-98d1-a69d94df310e</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">insert_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
		},
	},
];
</code></pre>