from annadb.data_types.journal import Journal
from jinja2 import Environment


def insert_data(connection):
    query = """
    collection|users|:insert[
        m{
          s|name|: s|John|,
          s|age|: n|30|,
          s|address|: m{
            s|street|: s|Park Avenue|,
            s|city|: s|New York|,
          },
          s|emails|: v[s|john@gmail.com|, s|john_ny@outlook.com|]
        },
        m{
          s|name|: s|Mary|,
          s|age|: n|25|,
          s|address|: m{
            s|street|: s|Rodeo Drive|,
            s|city|: s|Los Angeles|,
          },
          s|emails|: v[s|mary@gmail.com|, s|mary_la@outlook.com|]
        },
    ]
    """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def keep_projections(connection):
    query = """
    collection|users|:q[
        find[],
        sort[asc(value|name|)],
        project{
            s|name|:keep,
            s|age|:keep,
        }
    ]
    """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def existing_values_projections(connection):
    query = """
    collection|users|:q[
        find[],
        sort[asc(value|name|)],
        project{
            s|username|:value|name|
        }
    ]
    """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def primitive_values_projections(connection):
    query = """
    collection|users|:q[
        find[],
        sort[asc(value|name|)],
        project{
            s|title|:s|Dr. |,
        }
    ]
    """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def map_values_projections(connection):
    query = """
    collection|users|:q[
        find[],
        sort[asc(value|name|)],
        project{
            s|passport|:m{
                s|name|:value|name|,
            },
            s|address|:m{
                s|street|:keep
            }
        }
    ]
    """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def vector_values_projections(connection):
    query = """
    collection|users|:q[
        find[],
        sort[asc(value|name|)],
        project{
            s|name|:v[
                value|name|,
            ],
            s|emails|:v[s|TEST|,keep]
        }
    ]
    """
    input_data = Journal.deserialize(query)
    output_data = connection.send_query(query, value_only=False)
    return input_data.to_html(), output_data.to_html()


def build_0_2(connection):
    insert_in, insert_out = insert_data(connection)
    keep_in, keep_out = keep_projections(connection)
    existing_in, existing_out = existing_values_projections(connection)
    primitive_in, primitive_out = primitive_values_projections(connection)
    map_in, map_out = map_values_projections(connection)
    vector_in, vector_out = vector_values_projections(connection)

    env = Environment()
    with open("build/release_notes/0.2.0.md", "w") as output:
        with open("docs_gen/templates/release_notes/0.2.0.md", "r") as f:
            template = env.from_string(f.read())
            output.write(
                template.render(
                    insert_in=insert_in,
                    insert_out=insert_out,
                    keep_in=keep_in,
                    keep_out=keep_out,
                    existing_in=existing_in,
                    existing_out=existing_out,
                    primitive_in=primitive_in,
                    primitive_out=primitive_out,
                    map_in=map_in,
                    map_out=map_out,
                    vector_in=vector_in,
                    vector_out=vector_out,
                )
            )
