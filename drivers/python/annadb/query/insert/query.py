import typing

from annadb.query.base import BaseQuery
from annadb.query.query_set import QuerySet
from annadb.query.types import Insert
from annadb.response import ErrorResponse

if typing.TYPE_CHECKING:
    from annadb.collection import Collection


class InsertQuery(BaseQuery):
    def __init__(self, *args, collection: "Collection"):
        self.collection = collection
        self.query_set = QuerySet(
            Insert(*args), collection=self.collection.name
        )


class InsertOneQuery(InsertQuery):
    def __init__(self, item, collection: "Collection"):
        super(InsertOneQuery, self).__init__(item, collection=collection)

    def run(self):
        res = super(InsertOneQuery, self).run()
        if not isinstance(res, ErrorResponse):
            res["data"] = res.data[0]
        return res
