from jinja2 import Environment

from annadb.data_types.journal import Journal


def find(connection):
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
        collection|test|:find[
            gt{
                value|num|:n|4|
            }
        ]
        """

    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def build_find(connection):
    (find_in, find_out) = find(connection)

    env = Environment()
    with open("build/documentation/find.md", "w") as output:
        with open("docs_gen/templates//documentation/find.md", "r") as f:
            template = env.from_string(f.read())
            output.write(
                template.render(
                    find_in=find_in,
                    find_out=find_out,
                )
            )
