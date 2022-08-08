from urllib.parse import urlparse

import zmq

from annadb.collection import Collection
from annadb.dump import to_str
from annadb.data_types.journal import Journal
from annadb.transaction import Transaction

context = zmq.Context()


class Connection:
    def __init__(self, user_name: str, password: str, host: str, port: int):
        self.user_name = user_name
        self.password = password
        self.host = host
        self.port = port

        self.socket = context.socket(zmq.REQ)
        self.socket.connect(f"tcp://{host}:{port}")

    def send_query(self, *args, value_only: bool = True) -> Journal:
        data = []
        for query_set in args:
            if isinstance(query_set, str):
                if query_set.endswith(";"):
                    query_set = query_set[:-1]
                data.append(query_set)
            else:
                data.append(to_str(query_set))
        data = ";".join(data)
        self.socket.send_string(data)
        raw_response = self.socket.recv_string()
        response = Journal.deserialize(raw_response)
        if value_only:
            return response[0].value
        return response

    @classmethod
    def from_connection_string(cls, uri: str) -> "Connection":
        res = urlparse(uri)
        return cls(
            user_name=res.username,
            password=res.password,
            host=res.hostname,
            port=res.port,
        )

    def close(self):
        self.socket.close()

    def __getitem__(self, item) -> Collection:
        return Collection(name=item, conn=self)

    def new_transaction(self) -> Transaction:
        return Transaction(self)
