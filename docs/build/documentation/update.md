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
			<span class="prefix_link">test</span>|<span class="value_link">4339ace2-9ab3-4c79-b557-f9b78d66b7f9</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">3677c916-ac4d-40ab-89f4-def1e565e7ab</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">5ff00377-34ac-43b9-8ebb-71bb5ff78ebf</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">7bdd7c8f-e9da-42f6-b473-5a6fd9a1c90f</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
		},
	},
];
</code></pre>