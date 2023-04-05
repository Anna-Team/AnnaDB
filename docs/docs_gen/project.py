from jinja2 import Environment

from annadb.data_types.journal import Journal


def project(connection):
    collection = connection["test"]
    objects = [
        {
            "name": f"test_{i}",
            "num": i,
            "bool": True,
            "vec": [1, 2, 3],
            "map": {"bar": "baz"}
        } for i in range(3)
    ]
    collection.insert(
        *objects
    ).run()

    query = """
        collection|test|:q[
            find[],
            project{
                s|name|:s|foo|,
                s|num|:keep,
                s|vec|:v[keep, n|1|],
                s|map|:m{s|bar|:keep, s|test|:s|test|},
                s|new_field|:s|new_value|
            }
        ]
        """

    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def build_project(connection):
    (find_in, find_out) = project(connection)

    env = Environment()
    with open("build/documentation/project.md", "w") as output:
        with open("docs_gen/templates//documentation/project.md", "r") as f:
            template = env.from_string(f.read())
            output.write(
                template.render(
                    find_in=find_in,
                    find_out=find_out,
                )
            )
