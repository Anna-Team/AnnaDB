from jinja2 import Environment

from annadb.data_types.journal import Journal


def pipeline():
    query = """
    collection|test|:q[
        find[
        ],
        sort[
            asc(value|num|),
        ],
        limit(n|5|),
    ];
    """
    return Journal.deserialize(query).to_html()


def single_step():
    query = """
        collection|test|:find[
            gt{
                value|num|:n|4|,
            },
        ];
        """
    return Journal.deserialize(query).to_html()

def transaction():
    query = """
            collection|test|:q[
                find[gt{
                    value|num|:n|4|,
                },],
                update[
                    set{value|blink2.a|:n|100|}
                ]
            ];
            collection|test|:q[
                find[],
                update[
                    set{value|blink2.e|:n|1000|}
                ]
            ];
            """
    return Journal.deserialize(query).to_html()


def build_intro():
    env = Environment()
    with open("../build/documentation/introduction.md", "w") as output:
        with open("templates/documentation/introduction.md", "r") as f:
            template = env.from_string(f.read())
            output.write(
                template.render(
                    pipeline=pipeline(),
                    single_step_pipeline=single_step(),
                    transaction=transaction()
                )
            )
