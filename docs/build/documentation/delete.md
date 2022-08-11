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
			<span class="prefix_link">test</span>|<span class="value_link">4daa39d0-46d6-4962-a65b-479ceee1c765</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">dbe8b09f-3cb0-45f8-92fc-87e948d2ea3c</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">de333ee9-70de-4ec0-a1ce-962adf8ccfaf</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">7a0309b9-5c84-4860-90d6-697a8cc1a929</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
		},
	},
];
</code></pre>