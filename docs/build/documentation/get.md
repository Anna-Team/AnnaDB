Get one or many objects by id (link)

*Prefix:* `get`

*Value:* Vector of links

*Can start the pipeline:* Yes

*Steps before:* find, get, sort, limit, offset

*Steps after:* find, get, sort, limit, offset, update, delete

## Examples

<pre><code><span class="prefix_primitive">collection</span>|<span class="value_primitive">test</span>|:<span class="prefix_vector">get</span>[
	<span class="prefix_link">test</span>|<span class="value_link">716153f3-8761-4cb8-a9da-da490a109ce9</span>|,
	<span class="prefix_link">test</span>|<span class="value_link">dff45aa0-6664-4597-ba81-1a3f5b455a18</span>|,
];
</code></pre>

Output:

<pre><code><span class="prefix_primitive">result</span>:<span class="prefix_vector">ok</span>[
	<span class="prefix_map">response</span>{
		<span class="prefix_string">s</span>|<span class="value_string">data</span>|:<span class="prefix_map">objects</span>{
			<span class="prefix_link">test</span>|<span class="value_link">716153f3-8761-4cb8-a9da-da490a109ce9</span>|:<span class="prefix_string">s</span>|<span class="value_string">foo</span>|,
			<span class="prefix_link">test</span>|<span class="value_link">dff45aa0-6664-4597-ba81-1a3f5b455a18</span>|:<span class="prefix_string">s</span>|<span class="value_string">bar</span>|,
		},
		<span class="prefix_string">s</span>|<span class="value_string">meta</span>|:<span class="prefix_map">get_meta</span>{
			<span class="prefix_string">s</span>|<span class="value_string">count</span>|:<span class="prefix_number">n</span>|<span class="value_number">2</span>|,
		},
	},
];
</code></pre>