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
			<span class="prefix_link">test</span>|<span class="value_link">1c2f0dad-59ce-4b30-9944-549c59ff58b2</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">cc6c83a4-29a3-473b-bff7-42b24a8e14a8</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">3bbfe833-f2c9-4bbb-b883-17c5776c8bba</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">f9a0d990-22d0-49d6-a12e-cb20d166f755</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
		},
	},
];
</code></pre>