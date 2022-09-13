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
			<span class="prefix_link">test</span>|<span class="value_link">a652def6-1ba7-4e29-b881-7e9f6c6cff4c</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">6abf34d6-2571-49c8-9a76-5d4d3471b140</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">1556c60e-7e5a-4ed5-bc49-16dc57b24b95</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">f645c679-9406-453e-8e35-d73a459db3a0</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
		},
	},
];
</code></pre>