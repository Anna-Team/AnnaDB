Find objects using comparison and logical operators

*Prefix:* `find`

*Value:* Vector of operators

*Can start the pipeline:* Yes

*Steps before:* find, get, sort, limit, offset

*Steps after:* find, get, sort, limit, offset, update, delete

## Comparison operators

- Equal - `eq{...}`
- Not equal - `neq{...}`
- Greater than - `gt{...}`
- Greater than or equeal - `gte{...}`
- Less than - `lt{...}`
- Less than or equal - `lte{...}`
- Less than or equal - `lte{...}`

## Logical operators

- And - `and[...]`
- Or - `or[...]`
- Not - `not(...)`

In order to compare the value of the object `root path` notation could be used:

```shell
eq{root: s|bar|}
```

In order to compare a field of the object `path to value` notation could be used:

```shell
eq{value|path.to.field|: s|bar|}
```

## Example

Input:

{{ find_in }}

Output:

{{ find_out }}