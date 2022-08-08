import typing

if typing.TYPE_CHECKING:
    from annadb.connection import Connection
    from annadb.data_types.journal import QuerySet


class Transaction:
    def __init__(self, conn: "Connection"):
        self.conn = conn
        self.steps = []

    def push(self, step: "QuerySet"):
        self.steps.append(step.query_set)

    def run(self):
        return self.conn.send_query(*self.steps)
