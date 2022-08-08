import typing
from typing import Optional

from annadb.query.base import FindInterface
from annadb.query.query_set import QuerySet
from annadb.query.types import Find

if typing.TYPE_CHECKING:
    from annadb.collection import Collection


class FindQuery(FindInterface):
    def __init__(
        self,
        *args,
        collection: "Collection",
        query_set: Optional[QuerySet] = None
    ):
        self.collection = collection
        query = Find(*args)
        if query_set is None:
            self.query_set = QuerySet(query, collection=self.collection.name)
        else:
            query_set.push(query)
            self.query_set = query_set
