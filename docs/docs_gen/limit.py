from jinja2 import Environment

from annadb.data_types.journal import Journal


def limit(connection):
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
            sort[asc(value|num|)],
            limit(n|5|)
        ]
        """

    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def build_limit(connection):
    (limit_in, limit_out) = limit(connection)

    env = Environment()
    with open("../build/documentation/limit.md", "w") as output:
        with open("templates/documentation/limit.md", "r") as f:
            template = env.from_string(f.read())
            output.write(
                template.render(
                    limit_in=limit_in,
                    limit_out=limit_out,
                )
            )
