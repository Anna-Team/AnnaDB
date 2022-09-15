import click
import os
from rich.console import Console

from annadb.connection import Connection
from annadb.data_types.journal import Journal


@click.command()
@click.option(
    "--uri",
    required=True,
    type=str,
    help="Connection string",
)
def shell(uri):
    conn = Connection.from_connection_string(uri)
    console = Console(tab_size=4)
    console.print(
        r"""
      ___           ___           ___           ___           ___           ___
     /\  \         /\__\         /\__\         /\  \         /\  \         /\  \
    /::\  \       /::|  |       /::|  |       /::\  \       /::\  \       /::\  \
   /:/\:\  \     /:|:|  |      /:|:|  |      /:/\:\  \     /:/\:\  \     /:/\:\  \
  /::\~\:\  \   /:/|:|  |__   /:/|:|  |__   /::\~\:\  \   /:/  \:\__\   /::\~\:\__\
 /:/\:\ \:\__\ /:/ |:| /\__\ /:/ |:| /\__\ /:/\:\ \:\__\ /:/__/ \:|__| /:/\:\ \:|__|
 \/__\:\/:/  / \/__|:|/:/  / \/__|:|/:/  / \/__\:\/:/  / \:\  \ /:/  / \:\~\:\/:/  /
      \::/  /      |:/:/  /      |:/:/  /       \::/  /   \:\  /:/  /   \:\ \::/  /
      /:/  /       |::/  /       |::/  /        /:/  /     \:\/:/  /     \:\/:/  /
     /:/  /        /:/  /        /:/  /        /:/  /       \::/__/       \::/__/
     \/__/         \/__/         \/__/         \/__/         ~~            ~~
    """
    )

    while True:

        line = input(">>> ")
        if line:
            query = f"{line}\n"
            while line:
                line = console.input()
                query += f"{line}\n"

            if query == "":
                break

            try:
                request_doc = Journal.deserialize(query)
            except ValueError as e:
                console.print(str(e))
                continue

            os.system("cls" if os.name == "nt" else "clear")

            request_doc.pretty("Request", console)

            response_doc = conn.send_query(query, value_only=False)

            response_doc.pretty("Response", console)
        else:
            break

    conn.close()


if __name__ == "__main__":
    shell()
