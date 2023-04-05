from jinja2 import Environment

from annadb.data_types.journal import Journal


def sort(connection):
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

    query = f"""
        collection|test|:q[
            find[],
            sort[asc(value|num|)]
        ]
        """

    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def build_sort(connection):
    (sort_in, sort_out) = sort(connection)

    env = Environment()
    with open("build/documentation/sort.md", "w") as output:
        with open("docs_gen/templates//documentation/sort.md", "r") as f:
            template = env.from_string(f.read())
            output.write(
                template.render(
                    sort_in=sort_in,
                    sort_out=sort_out,
                )
            )
