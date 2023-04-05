from jinja2 import Environment

from annadb.data_types.journal import Journal


def insert(connection):
    query = """
    collection|test|:insert[
        s|foo|,
        n|100|,
        b|true|,
        v[n|1|, n|2|, n|3|],
        m{s|bar|:s|baz|}
    ]
    """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def build_insert(connection):
    (insert_in, insert_out) = insert(connection)

    env = Environment()
    with open("build/documentation/insert.md", "w") as output:
        with open("docs_gen/templates//documentation/insert.md", "r") as f:
            template = env.from_string(f.read())
            output.write(
                template.render(
                    insert_in=insert_in,
                    insert_out=insert_out,
                )
            )
