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
			<span class="prefix_link">test</span>|<span class="value_link">52abe612-43b5-4010-9d8a-4f1206f867be</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">f1a5cd85-13cc-4f95-a8de-fab9323f0ab7</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">98b1984f-d2a6-42da-a7e7-2dcad00d94f1</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">9b6ae250-92f8-455d-a38d-13aacd047044</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">49e78d32-18c8-4202-ac94-4d70ca9c4b57</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">insert_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
		},
	},
];
</code></pre>