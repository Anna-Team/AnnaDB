Limit number of found objects

*Prefix:* `limit`

*Value:* Modifier with a number

*Can start the pipeline:* No

*Steps before:* find, get, sort, limit, offset

*Steps after:* find, get, sort, limit, offset, update, delete

## Example

Input:

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">test</span>|:<span class="prefix_vector">q</span>[
	<span class="prefix_vector">find</span>[
	],
	<span class="prefix_vector">sort</span>[
		<span class="prefix_modifier">asc</span>(<span class="prefix_primitive">value</span>|<span class="value_primitive">num</span>|),
	],
	<span class="prefix_modifier">limit</span>(<span class="prefix_number">n</span>|<span class="value_number">5</span>|),
];
</code></pre>

Output:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_map">objects</span>{
			<span class="prefix_link">test</span>|<span class="value_link">fc239ae5-45b2-47d8-bca1-47f671137c71</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">0</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_0</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">01324499-cabf-4e81-bb39-e2b648270d01</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_1</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">daa7cda1-be29-4cd9-9aaf-9ec860f4278b</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_2</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">e22a44af-df8c-4661-a42e-271e862bed6a</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">3</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_3</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">7a7b2cfb-6afc-451a-b119-e71d929c7d5b</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_4</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">4</span>|,
			},
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">find_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">5</span>|,
		},
	},
];
</code></pre>