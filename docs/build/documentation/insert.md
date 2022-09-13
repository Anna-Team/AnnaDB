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
			<span class="prefix_link">test</span>|<span class="value_link">7f20f778-9688-409c-98c6-03bc311e4c97</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">cbf9386b-0117-4261-a227-d456fb84159c</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">3554c97b-2c9b-4b4a-94a8-5b386ce0bf59</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">e7989736-f13a-49b2-b003-64bee1d0aa0a</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">1517c986-3afd-4a33-8d74-95c9795caec1</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">insert_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
		},
	},
];
</code></pre>