import typing
from typing import Optional

from annadb.query.base import FindInterface
from annadb.query.query_set import QuerySet
from annadb.query.types import Get
from annadb.response import ErrorResponse

if typing.TYPE_CHECKING:
    from annadb.collection import Collection


class GetQuery(FindInterface):
    def __init__(
        self,
        *args,
        collection: "Collection",
        query_set: Optional[QuerySet] = None
    ):
        self.collection = collection
        query = Get(*args)
        if query_set is None:
            self.query_set = QuerySet(query, collection=self.collection.name)
        else:
            query_set.push(query)
            self.query_set = query_set


class GetOneQuery(GetQuery):
    def __init__(self, id, collection: "Collection"):
        super(GetOneQuery, self).__init__(id, collection=collection)

    def run(self):
        res = super(GetOneQuery, self).run()
        if not isinstance(res, ErrorResponse):
            res["data"] = res.data.values()[0] if len(res.data) > 0 else None
        return res
