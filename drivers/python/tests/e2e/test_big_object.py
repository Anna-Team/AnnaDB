import time

# import pytest

from annadb.data_types.journal import QuerySet
from annadb.response import OkResponse
from annadb.query.types import Insert


# @pytest.mark.skip("It is a long test")
class TestInsertBig:
    def test_insert_one(self, conn):
        lst = [i for i in range(100000)]
        q = QuerySet(Insert(lst), collection="test_big")
        start = time.time()
        res = conn.send_query(q)
        end = time.time()
        assert end - start < 5
        assert isinstance(res, OkResponse)

        start = time.time()
        res = conn.send_query(
            """
        collection|test_big|:find[]
        """
        )
        end = time.time()
        assert end - start < 5
        assert isinstance(res, OkResponse)

    def test_insert_many_primitives(self, conn):
        lst = [i for i in range(100000)]
        q = QuerySet(Insert(*lst), collection="test_big")
        start = time.time()
        res = conn.send_query(q)
        end = time.time()
        assert end - start < 5
        print(end - start)
        assert isinstance(res, OkResponse)

        start = time.time()
        res = conn.send_query(
            """
        collection|test_big|:find[]
        """
        )
        end = time.time()
        assert end - start < 5
        assert isinstance(res, OkResponse)
        assert res[0]["meta"]["count"] == 100000

    def test_insert_many_maps(self, conn):
        lst = [
            {
                "foo_1": "bar_1",
                "foo_2": "bar_2",
                "foo_3": "bar_3",
                "foo_4": "bar_4",
                "foo_5": "bar_5",
            }
            for _ in range(10000)
        ]
        q = QuerySet(Insert(*lst), collection="test_big")
        start = time.time()
        res = conn.send_query(q)
        end = time.time()
        assert end - start < 5
        assert isinstance(res, OkResponse)

        start = time.time()
        res = conn.send_query(
            """
        collection|test_big|:find[]
        """
        )
        end = time.time()
        assert end - start < 5
        assert isinstance(res, OkResponse)
        assert res[0]["meta"]["count"] == 10000

    def test_find_many_maps(self, conn):
        lst = [
            {
                "foo_1": i,
                "foo_2": 2,
                "foo_3": 2,
                "foo_4": 2,
                "foo_5": 2,
            }
            for i in range(100000)
        ]
        q = QuerySet(Insert(*lst), collection="test_big")
        conn.send_query(q)

        start = time.time()
        res = conn.send_query(
            """
        collection|test_big|:find[gte{value|foo_1|: n|95000|}]
        """
        )
        end = time.time()
        assert end - start < 5
        assert isinstance(res, OkResponse)
        assert res[0]["meta"]["count"] == 5000
