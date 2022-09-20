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
			<span class="prefix_link">test</span>|<span class="value_link">20eefe75-7e29-4b1e-96c1-8fbeb30ac489</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">0f90aeba-a0b9-4dfa-ad81-e7e8e88e9d15</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">276a7f85-fe2a-4b87-abe7-62363dfd9d65</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">d6525ef8-f233-4882-b349-ef7646cdba3d</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">af2a01e7-75a1-497c-8272-0a2fe7d9d307</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">insert_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
		},
	},
];
</code></pre>