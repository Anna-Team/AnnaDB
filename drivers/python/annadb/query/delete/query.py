import typing

from annadb.query.base import BaseQuery
from annadb.query.query_set import QuerySet
from annadb.query.types import Delete

if typing.TYPE_CHECKING:
    from annadb.collection import Collection


class DeleteQuery(BaseQuery):
    def __init__(
        self,
        collection: "Collection",
        query_set: typing.Optional[QuerySet] = None,
    ):
        self.collection = collection
        query = Delete()
        if query_set is None:
            self.query_set = QuerySet(query, collection=self.collection.name)
        else:
            query_set.push(query)
            self.query_set = query_set
