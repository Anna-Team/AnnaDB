Remove found objects or the whole collection.

*Prefix:* `delete`

*Value:* no value. Prefix-only primitive

*Can start the pipeline:* No

*Steps before:* find, get, sort, limit, offset

*Steps after:* -

## Example

Input:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">test</span>|:<span class="prefix_vector">q</span>[
	<span class="prefix_vector">find</span>[
		<span class="prefix_map">gt</span>{
			<span class="prefix_primitive">value</span>|<span class="value_primitive">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
		},
	],
	<span class="prefix_primitive">delete</span>,
];
</code></pre>

Output:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_vector">ids</span>[
			<span class="prefix_link">test</span>|<span class="value_link">c613554f-4fab-4dd9-a45e-a24a88d062d1</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">4fa07f87-0a9c-4ac8-a0af-c4aa09ecf2e6</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">a3b2efd0-594d-4c42-9747-af595a664ade</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">9e99265a-1dd8-40b1-94de-d6096f0baf99</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
		},
	},
];
</code></pre>