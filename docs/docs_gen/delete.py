from jinja2 import Environment

from annadb.data_types.journal import Journal


def delete(connection):
    collection = connection["test"]
    objects = [
        {
            "name": f"test_{i}",
            "num": i
        } for i in range(10)
    ]
    collection.insert(
        *objects
    ).run()

    query = """
        collection|test|:q[
            find[gt{value|num|: n|5|}],
            delete,
        ]
        """

    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def build_delete(connection):
    (delete_in, delete_out) = delete(connection)

    env = Environment()
    with open("build/documentation/delete.md", "w") as output:
        with open("docs_gen/templates//documentation/delete.md", "r") as f:
            template = env.from_string(f.read())
            output.write(
                template.render(
                    delete_in=delete_in,
                    delete_out=delete_out,
                )
            )
