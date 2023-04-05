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
			<span class="prefix_link">test</span>|<span class="value_link">0b7e9bb7-53b5-4e91-b4f8-09ffd78fb585</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">0</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_0</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">34310392-1be2-4b9e-8474-9a63cca9104a</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_1</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">1</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">d373fa90-0574-4312-8517-8d9c5d617dfb</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_2</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">fb61ecdc-ad4b-45a5-8da1-7076ce0fb0f0</span>|:<span class="prefix_map">m</span>{
				<span class="prefix_string">s</span>|<span class="value_string">num</span>|:<span class="prefix_number">n</span>|<span class="value_number">3</span>|,
				<span class="prefix_string">s</span>|<span class="value_string">name</span>|:<span class="prefix_string">s</span>|<span class="value_string">test_3</span>|,
			},
			<span class="prefix_link">test</span>|<span class="value_link">c1c7dd26-4fbb-43d2-ba92-f38401a992be</span>|:<span class="prefix_map">m</span>{
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