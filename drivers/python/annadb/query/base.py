from annadb.response import ErrorResponse


class BaseQuery:
    def run(self):
        res = self.collection.conn.send_query(self.query_set)
        if not isinstance(res, ErrorResponse):
            res = res[0]
        return res


class FindInterface(BaseQuery):
    def get(self, *args):
        from annadb.query.get.query import GetQuery

        return GetQuery(
            *args, query_set=self.query_set, collection=self.collection
        )

    def find(self, *args):
        from annadb.query.find.query import FindQuery

        return FindQuery(
            *args, query_set=self.query_set, collection=self.collection
        )

    def delete(self):
        from annadb.query.delete.query import DeleteQuery

        return DeleteQuery(
            query_set=self.query_set, collection=self.collection
        )

    def sort(self, *args):
        from annadb.query.sort.query import SortQuery

        return SortQuery(
            *args, query_set=self.query_set, collection=self.collection
        )

    def limit(self, number: int):
        from annadb.query.limit.query import LimitQuery

        return LimitQuery(
            number, query_set=self.query_set, collection=self.collection
        )

    def offset(self, number: int):
        from annadb.query.offset.query import OffsetQuery

        return OffsetQuery(
            number, query_set=self.query_set, collection=self.collection
        )

    def update(self, *args):
        from annadb.query.update.query import UpdateQuery

        return UpdateQuery(
            *args, query_set=self.query_set, collection=self.collection
        )
