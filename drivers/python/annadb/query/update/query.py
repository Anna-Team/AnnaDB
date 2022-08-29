import typing

from annadb.query.base import BaseQuery
from annadb.query.query_set import QuerySet
from annadb.query.types import Update

if typing.TYPE_CHECKING:
    from annadb.collection import Collection


class UpdateQuery(BaseQuery):
    def __init__(self, *args, query_set: QuerySet, collection: "Collection"):
        self.collection = collection
        query_set.push(Update(*args))
        self.query_set = query_set
