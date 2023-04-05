In this tutorial I'll create a database for the candy store to show the basic AnnaDB features.

## Insert primitive

let's start with categories. Let's represent categories as simple string objects. To do so let's insert the first one into the `categories` collection.

**Request**:

{{ insert_category_in }}

`collection|categories|` shows on which collection the query will be applied. In our case - `categories`

`insert[...]` - is a query step. You can insert one or many objects using the `insert` operation.

`s|sweets|` - is the object to insert. In this case, it is a string primitive. Prefix `s` means that it is a string, `|` wrap the value of the primitive. Other primitive types could be found in the [Data Types section](../../documentation/data_types/).

**Response**:

{{ insert_category_out }}

If everything is ok, in result will be returned `ok[...]` vector with responses for all the transaction pipelines. Each response contains `data` and `meta` information. In our case there is only one response with a vector of `ids` in `data` and a number of inserted objects in `meta`

## Insert container

Let's insert a more complicated object now - a chocolate bar. It will have fields:

- name
- price
- category

For the category, I'll use the already created one.

**Request**:

{{ insert_chocolate_bar_in }}

The query is similar to the previous one, but the object is not a primitive, but a map. The value of the `category` field is a link, that was received after the previous insert.

**Response**:

{{ insert_chocolate_bar_out }}

The response is nearly the same as before - link in data and number of inserted objects in meta.

## Get object

Let's retrieve the information about this chocolate bar now. I'll use the `get` operation for this, to the object by id

**Request**:

{{ get_chocolate_bar_in }}

This time I use the `get[...]` query step. Using this step you can retrieve one or many objects using object links.

**Response**:

{{ get_chocolate_bar_out }}

In the response here you can see the `objects{...}` map, where keys are links to objects and values are objects. `objects{}` map keeps the order - it will return objects in the same order as they were requested in the get step, or as they were sorted by the sort step.

The category was fetched automatically and the value was returned.

Let's insert another chocolate bar there to have more objects in the collection:

{{ mars_insert_in }}

I use the same category id for this bar.

## Modify primitive

Let's modify the category to make it more accurate.

**Request**:

{{ update_category_in }}

The query here consists of 2 steps. `Get the object by link` step and `modify this object` step. The `update[...]` operation is a vector of update operators. [Read more about the update](../../documentation/update/).

**Response**:

{{ update_category_out }}

The response of the update operation contains the ids of the updated objects as data and the number of the updated objects as meta.

Let's take a look, at how this affected the chocolate objects.

**Request**:

{{ find_chocolate_in }}

To find objects I use the `find[...]` operation. It is a vector of find operators. If it is empty, all the collection objects will be returned.

**Response**:

{{ find_chocolate_out }}

The category was changed for both products, as the category object was linked with these objects.

## Modify container

Now I'll increase the price of the bars, where it is less than 2

**Request**:

{{ update_price_in }}

The `find` step can stay before the `update` step as well. All the found objects will be updated. Read more about `find` operation and operators [here](../../documentation/find/).

**Response**:

{{ update_price_out }}

The response is similar to the previous one.

Here is how all the products look like after update:

{{ find_chocolate_out_2 }}

## Sort objects

To sort objects I'll use the `sort` operation against the price field

**Request**:

{{ sort_in }}

The `sort[...]` operation is a vector of sort operators - `asc` and `desc`. Sort operators are modifiers, that contain paths to the sorting value. The `sort` operation is not an independent step, it can stay only after find-like operations, that return objects. You can read more about sort [here](../../documentation/update/)

**Response**:

{{ sort_out }}

Objects in the response are sorted by price now.

It is useful to use `limit` and `offset` operations together with sort. You can read about them in the [documentation](../../documentation/limit/)

## Make projections

To get only the name and price fields I'll use the `project` operation

**Request**:

{{ projections_keep_in }}

The `keep` operator is used to keep the value of the field or subfield in the output.

**Response**:

{{ projections_keep_out }}

To set a new field with already existing value I use `root` operator

**Request**:

{{ projections_existing_values_in }}

**Response**:

{{ projections_existing_values_out }}

It is possible to set any value to the field as well

**Request**:

{{ projections_new_values_in }}

**Response**:

{{ projections_new_values_out }}

## Delete objects

You can use `delete` operation after any find-like step to delete all the found objects. Or it can be used independently to delete the whole collection.

**Request**:

{{ delete_in }}

The `delete` operation is a primitive without value.

**Response**:

{{ delete_out }}

The response contains affected ids in `data` and the number of deleted objects in `meta`
