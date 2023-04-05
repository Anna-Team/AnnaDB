Update values of found objects

*Prefix:* `update`

*Value:* Vector of update operators

*Can start the pipeline:* No

*Steps before:* find, get, sort, limit, offset

*Steps after:* -

## Operators:

- Inc - `inc{...}`
- Set - `set{...}`

## Example

Input:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">test</span>|:<span class="prefix_vector">q</span>[
	<span class="prefix_vector">find</span>[
		<span class="prefix_map">gt</span>{
			<span class="prefix_primitive">value</span>|<span class="value_primitive">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
		},
	],
	<span class="prefix_vector">update</span>[
		<span class="prefix_map">set</span>{
			<span class="prefix_primitive">value</span>|<span class="value_primitive">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">1000</span>|,
		},
	],
];
</code></pre>

Output:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_vector">ids</span>[
			<span class="prefix_link">test</span>|<span class="value_link">5bd5c092-2536-4f30-b139-a6f2ae653789</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">8b55d573-b881-4f6f-8da9-f687513b7e91</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">5361f98a-8d4b-43db-a08f-6c558ea173f1</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">9e529dc7-0773-4cd7-9d3b-de283a25b1d8</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
		},
	},
];
</code></pre>