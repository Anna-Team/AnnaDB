import typing

from annadb.query.delete.query import DeleteQuery
from annadb.query.find.query import FindQuery
from annadb.query.get.query import GetQuery, GetOneQuery
from annadb.query.insert.query import InsertQuery, InsertOneQuery

if typing.TYPE_CHECKING:
    from annadb.connection import Connection


class Collection:
    def __init__(self, name: str, conn: "Connection"):
        self.name = name
        self.conn = conn

    def get(self, *args):
        return GetQuery(*args, collection=self)

    def get_one(self, id):
        return GetOneQuery(id, collection=self)

    def find(self, *args):
        return FindQuery(*args, collection=self)

    def all(self):
        return FindQuery(collection=self)

    def insert(self, *args):
        return InsertQuery(*args, collection=self)

    def insert_one(self, item):
        return InsertOneQuery(item, collection=self)

    def delete(self):
        return DeleteQuery(collection=self)
