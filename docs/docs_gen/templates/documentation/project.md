Modify the output data using projection template

*Prefix:* `project`

*Value:* Map of fields

*Can start the pipeline:* No

*Steps before:* find, get, sort, limit, offset

*Steps after:* -

## Operators

- Keep - `keep` - keep the respective value

## Usage

In order to keep a field or subfield of the object `keep` operator could be used:

```shell

s|field|:keep,
s|map|:m{
    s|field_2|:keep,
},

```

To set an existing value to a new field `value` notation could be used:

```shell

s|new_field|:value|field|,

```

To set a new value to a field primitives, maps or vectors could be used:

```shell

s|field_1|:s|new value|,

s|field_2|:m{
    s|field|:s|new value|,
},

s|field_3|:v[
    s|new value|,
],

```


## Example

Input:

{{ find_in }}

Output:

{{ find_out }}
