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
			<span class="prefix_link">test</span>|<span class="value_link">05de53aa-5e84-4d62-8ac5-972697a3fe9b</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">69d99075-a544-4242-accf-3dfe085e6f1c</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">d7b3d4f7-78e0-4686-bd54-21d68b8b9160</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">685204d5-e8ef-415a-afd9-1072c76156ac</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
		},
	},
];
</code></pre>