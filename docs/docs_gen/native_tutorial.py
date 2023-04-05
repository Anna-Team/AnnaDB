from jinja2 import Environment

from annadb.data_types.journal import Journal
from annadb.dump import to_str


def insert_category(connection):
    query = f"""
        collection|categories|:insert[s|sweets|]
        """

    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html(), \
           output_data[0].value[0].data[0]


def insert_chocolate_bar(connection, category_id):
    query = f"""
    collection|products|:insert[
        m{{
            s|name|:s|Tony's|,
            s|price|:n|5.95|,
            s|category|:{to_str(category_id)}
        }}
    ]
    """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html(), \
           output_data[0].value[0].data[0]


def get_bar(connection, bar_id):
    query = f"""
        collection|products|:get[
            {to_str(bar_id)}
        ]
        """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def insert_mars(connection, category_id):
    query = f"""
        collection|products|:insert[
            m{{
                s|name|:s|Mars|,
                s|price|:n|2|,
                s|category|:{to_str(category_id)}
            }}
        ]
        """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html(), \
           output_data[0].value[0].data[0]


def update_category(connection, category_id):
    query = f"""
        collection|categories|:q[
            get[{to_str(category_id)}],
            update[set{{root: s|chocolate|}}]
        ]
        """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def find_chocolate(connection):
    query = f"""
        collection|products|:find[]
        """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def update_price(connection):
    query = f"""
            collection|products|:q[
                find[lt{{value|price|:n|3|}}],
                update[inc{{value|price|: n|2|}}]
            ]
            """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def sort_bars(connection):
    query = f"""
            collection|products|:q[
                find[],
                sort[asc(value|price|)]
            ]
            """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def delete_bars(connection):
    query = f"""
                collection|products|:q[
                    find[gt{{value|price|:n|5|}}],
                    delete
                ]
                """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def projections_keep(connection):
    query = """
                collection|products|:q[
                    find[],
                    project{s|name|: keep, s|price|: keep}
                ]
                """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def projections_existing_values(connection):
    query = """
                collection|products|:q[
                    find[],
                    project{s|new_field|: value|category|}
                ]
                """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def projections_new_values(connection):
    query = """
                collection|products|:q[
                    find[],
                    project{s|new_field|: s|new_value|}
                ]
                """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def build_native_tutorial(connection):
    (insert_category_in, insert_category_out, category_id) = insert_category(
        connection)
    (insert_chocolate_bar_in, insert_chocolate_bar_out,
     bar_id) = insert_chocolate_bar(
        connection, category_id)

    (get_chocolate_bar_in, get_chocolate_bar_out) = get_bar(
        connection, bar_id)

    (mars_insert_in, mars_insert_out,
     mars_id) = insert_mars(
        connection, category_id)

    (update_category_in, update_category_out) = update_category(connection,
                                                                category_id)

    (find_chocolate_in, find_chocolate_out) = find_chocolate(connection)

    (update_price_in, update_price_out) = update_price(connection)

    (find_chocolate_in_2, find_chocolate_out_2) = find_chocolate(connection)
    (sort_in, sort_out) = sort_bars(connection)
    (delete_in, delete_out) = delete_bars(connection)
    (projections_keep_in, projections_keep_out) = projections_keep(connection)
    (projections_existing_values_in, projections_existing_values_out) = projections_existing_values(connection)
    (projections_new_values_in, projections_new_values_out) = projections_new_values(connection)

    env = Environment()
    with open("build/tutorial/native/index.md", "w") as output:
        with open("docs_gen/templates//tutorial/native/index.md", "r") as f:
            template = env.from_string(f.read())
            output.write(
                template.render(
                    insert_category_in=insert_category_in,
                    insert_category_out=insert_category_out,
                    insert_chocolate_bar_in=insert_chocolate_bar_in,
                    insert_chocolate_bar_out=insert_chocolate_bar_out,
                    get_chocolate_bar_in=get_chocolate_bar_in,
                    get_chocolate_bar_out=get_chocolate_bar_out,
                    mars_insert_in=mars_insert_in,
                    update_category_in=update_category_in,
                    update_category_out=update_category_out,
                    find_chocolate_in=find_chocolate_in,
                    find_chocolate_out=find_chocolate_out,
                    update_price_in=update_price_in,
                    update_price_out=update_price_out,
                    find_chocolate_out_2=find_chocolate_out_2,
                    sort_in=sort_in,
                    sort_out=sort_out,
                    delete_in=delete_in,
                    delete_out=delete_out,
                    projections_keep_in=projections_keep_in,
                    projections_keep_out=projections_keep_out,
                    projections_existing_values_in=projections_existing_values_in,
                    projections_existing_values_out=projections_existing_values_out,
                    projections_new_values_in=projections_new_values_in,
                    projections_new_values_out=projections_new_values_out,
                )
            )
