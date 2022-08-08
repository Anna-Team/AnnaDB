from annadb.constants import QUERY_SET
from annadb.data_types.vector import VectorBase


class QuerySet(VectorBase):
    prefix = QUERY_SET

    def __init__(self, *args, collection: str = ""):
        self.collection = collection
        super(QuerySet, self).__init__(*args)

        self.queries = list(args)

    def push(self, query):
        self.queries.append(query)
