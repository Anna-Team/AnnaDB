In this tutorial, I'll create a database for the candy store to show the basic
AnnaDB features.

## Connect

The `Connection` class is used to connect to the AnnaDB

```python
from annadb import Connection

conn = Connection.from_connection_string("annadb://localhost:10001")
```

## Insert primitive

let's start with categories. Let's represent categories as simple string
objects. To do so let's insert the first one into the `categories` collection.

**Request**:

```python
...

categories = conn["categories"]

response = categories.insert_one("sweets").run()
```

There are two insert operators - `insert` and `insert_one`.

- `insert` - operator to insert one or many objects. Response data is a list of
  links (ids). Meta - number of inserted items.
- `insert_one` - operator to insert a single object. Response data is a link.

Let's get the inserted id

```python
...

sweets_id = response.data
```

## Insert container

Let's insert a more complicated object now - a chocolate bar. It will have
fields:

- name
- price
- category

For the category, I'll use the already created one.

**Request**:

```python
products = conn["products"]

response = products.insert_one(
    {
        "name": "Tony's",
        "price": 5.95,
        "category": sweets_id
    }
)

tony_id = response.data
```

The query is similar to the previous one, but the object is not a str, but a
dict. The value of the `category` field is a link, that was received after the
previous insert.

## Get object

Let's retrieve the information about this chocolate bar now.

There are two ways to get objects by link: `get` and `get_one`. You can pass
any number of links to the `get` operator. It will return an ordered dict of
the link-object pars. `get_one` operator is used to get a single object.

**Request**:

```python
...
response = products.get_one(tony_id).run()

print(response.data["category"])

>> > sweets
```

The category was fetched automatically and the value was returned.

Let's insert another chocolate bar there to have more objects in the
collection:

```python
...

response = products.insert_one(
    {
        "name": "Mars",
        "price": 2,
        "category": sweets_id
    }
).run()

mars_id = response.data
```

I use the same category id for this bar.

## Modify primitive

Let's modify the category to make it more accurate.

**Request**:

```python
from annadb import Set, root

categories.get(sweets_id).update(Set({root: "chocolate"})).run()
```

The query here consists of 2 steps. `Get the object by link` step
and `modify this object` step.

The `root` object is a pointer to the value to update. For Vector and Map
object it works a starting point in the path like `root.path.to.value`

Let's take a look, at how this affected the chocolate objects.

**Request**:

```python
response = products.all().run()

for k, v in response.data.items():
    print(v["category"])

>>> chocolate
>>> chocolate
```

The category was changed for both products, as the category object was linked
with these objects.

## Modify container

Now I'll increase the price of the bars, where it is less than 2

```python
from annadb import Inc

products.find(root.price < 3).update(Inc({root.price: 2})).run()
```

The `find` step can stay before the `update` step as well. All the found
objects will be updated.

Let's check the prices now:

```python
...

response = products.all().run()

for k, v in response.data.items():
    print(v["name"], v["price"])

>>> Tony's 5.95
>>> Mars 4.0
```

## Sort objects

To sort objects I'll use the `sort` operation against the price field

**Request**:

```python
response = products.all().sort(+root.price).run()

for k, v in response.data.items():
    print(v["name"], v["price"])

>>> Mars 4.0
>>> Tony's 5.95
```

Objects in the response are sorted by price now.

It is useful to use `limit` and `offset` operations together with sort. You can
read about them in the [documentation](../../documentation/limit/)

## Make projections

To get only the name and price fields I'll use the `project` operation

**Request**:

```python
from annadb import keep

response = products.all().project({
    "name": keep,
    "price": keep
}).run()

```

The `keep` operator is used to keep the value of the field or subfield in the output.

To set a new field with already existing value I use `root` operator

**Request**:

```python
from annadb import root

response = products.all().project({
    "new_field": root.category
}).run()
```

It is possible to set any value to the field as well

**Request**:

```python
response = products.all().project({
    "new_field": "some value"
}).run()
```

## Delete objects

You can use the `delete` operation after any find-like step to delete all the found
objects. Or it can be used independently to delete the whole collection.

**Request**:

```python
products.find(root.price < 5).delete().run()
```
