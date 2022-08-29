import typing

from annadb.query.base import FindInterface
from annadb.query.query_set import QuerySet
from annadb.query.types import Offset

if typing.TYPE_CHECKING:
    from annadb.collection import Collection


class OffsetQuery(FindInterface):
    def __init__(
        self, number: int, query_set: QuerySet, collection: "Collection"
    ):
        self.collection = collection
        self.query_set = query_set
        self.query_set.push(Offset(number))
