Update values of found objects

*Prefix:* `update`

*Value:* Vector of update operators

*Can start the pipeline:* No

*Steps before:* find, get, sort, limit, offset

*Steps after:* -

## Operators:

- Inc - `inc(...)`
- Set - `set(...)`

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
			<span class="prefix_link">test</span>|<span class="value_link">ffab4f1f-31a4-4ca7-9410-6a57d307a142</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">edbc450d-9c69-4100-991d-a5cf92305542</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">78795e74-12ea-488b-a70a-24ef8b956ae3</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">0239c85f-ee69-4199-b307-29a78e18e41c</span>|,
		],
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">update_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
		},
	},
];
</code></pre>