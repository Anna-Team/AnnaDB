from jinja2 import Environment

from annadb.data_types.journal import Journal
from annadb.dump import to_str


def get(connection):
    collection = connection["test"]
    resp_insert = collection.insert("foo", "bar").run()
    ids = resp_insert.data

    query = f"""
        collection|test|:get[
            {to_str(ids[0])},
            {to_str(ids[1])}
        ]
        """

    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def build_get(connection):
    (get_in, get_out) = get(connection)

    env = Environment()
    with open("../build/documentation/get.md", "w") as output:
        with open("templates/documentation/get.md", "r") as f:
            template = env.from_string(f.read())
            output.write(
                template.render(
                    get_in=get_in,
                    get_out=get_out,
                )
            )
