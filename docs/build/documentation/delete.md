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
			<span class="prefix_link">test</span>|<span class="value_link">1895abc6-c0e3-4243-9421-1796265088db</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">c18a697c-c535-4347-8bf4-5c86cf168883</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">8cadf01e-4812-4515-affc-7e7191c248bf</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">633fd795-803b-46c8-8581-afe4989f2505</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
		},
	},
];
</code></pre>