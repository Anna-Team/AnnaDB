There are primitive and container data types in AnnaDB.

## Primitives

Primitive data types are a set of basic types whose values can not be
decoupled. In TySON, primitives are represented as `prefix|value|` or `prefix`
only. Prefix in AnnaDB shows the data type.

<table >
	<tbody>
		<tr>
			<td>Type</td>
			<td>Description</td>
			<td>Prefix</td>
			<td>Example</td>
		</tr>
		<tr>
			<td>Number</td>
			<td>Integer or float point number</td>
            <td>n</td>
			<td><code><span class="prefix_number">n</span>|<span class="value_number">101</span>|</code></td>
		</tr>
		<tr>
			<td>String</td>
			<td>Any string. `|` symbols must be escaped with `\`</td>
            <td>s</td>
			<td><code><span class="prefix_string">s</span>|<span class="value_string">Lorem ipsum</span>|</code></td>
		</tr>
		<tr>
			<td>Bool</td>
			<td>A boolean value</td>
			<td>b</td>
            <td><code><span class="prefix_bool">b</span>|<span class="true">true</span>|</code></td>
		</tr>
        <tr>
			<td>Null</td>
			<td>A marker that indicating that something has no value</td>
			<td>null</td>
            <td><code><span class="prefix_null">null</span></code></td>
		</tr>
		<tr>
			<td>Unix Timestamp </td>
			<td>The number of seconds that have elapsed since the Unix epoch</td>
			<td>uts</td>
			<td><code><span class="prefix_number">uts</span>|<span class="value_number">123456789</span>|</code> </td>
		</tr>
		<tr>
			<td>Link </td>
			<td>Id of an object. The collection name is used for the prefix</td>
			<td>Collection name </td>
			<td style="word-break:keep-all"><code><span class="prefix_link">users</span>|<span class="value_link">e0bbcda2-0911-495e-9f0f-ce00db489f10</span>|</code></td>
		</tr>
	</tbody>
</table>


## Containers

Container data types keep primitive and container objects using specific rules.

<table >
	<tbody>
		<tr>
			<td>Type</td>
			<td>Description</td>
			<td>Prefix</td>
			<td>Example</td>
		</tr>
		<tr>
			<td>Vector</td>
			<td>An ordered set of elements of any type</td>
            <td>v</td>
			<td><code><span class="prefix_vector">v</span>[<span class="prefix_number">n</span>|<span class="value_number">1</span>|,<span class="prefix_number">n</span>|<span class="value_number">2</span>|,<span class="prefix_number">n</span>|<span class="value_number">3</span>|,]</code></td>
		</tr>
		<tr>
			<td>Map</td>
			<td>An associative array</td>
            <td>m</td>
			<td><code><span class="prefix_map">m</span>{<span class="prefix_string">
  s</span>|<span class="value_string">bar</span>|:<span class="prefix_string">
  s</span>|<span class="value_string">baz</span>|,}</code></td>
		</tr>
	</tbody>
</table>
