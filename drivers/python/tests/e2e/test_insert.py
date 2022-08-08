from uuid import UUID

import pytest

from annadb.data_types.journal import QuerySet
from annadb.data_types.map import Map
from annadb.data_types.primitive import Link, String, Number, Bool, Null
from annadb.data_types.vector import Vector
from annadb.dump import to_str
from annadb.query.types import Insert


class TestInsertPrimitive:
    @pytest.mark.parametrize(
        "query,native_resp,conv_resp",
        [
            (
                """
        collection|test|: insert[
                              s|foo|,
                          ]
        """,
                "foo",
                String("foo"),
            ),
            (QuerySet(Insert("foo"), collection="test"), "foo", String("foo")),
            (
                """
        collection|test|: insert[
                    n|100|,
                ]
        """,
                100,
                Number(100),
            ),
            (QuerySet(Insert(100), collection="test"), 100, Number(100)),
            (
                """
        collection|test|: insert[
                    b|true|,
                ]
        """,
                True,
                Bool(True),
            ),
            (QuerySet(Insert(True), collection="test"), True, Bool(True)),
            (
                """
        collection|test|: insert[
                    null,
                ]
        """,
                None,
                Null(None),
            ),
            (QuerySet(Insert(None), collection="test"), None, Null(None)),
        ],
    )
    def test_insert_string(self, conn, query, native_resp, conv_resp):
        resp_insert = conn.send_query(query)
        assert type(resp_insert[0].data[0]) == Link
        assert type(resp_insert[0].data[0].value) == UUID
        assert resp_insert[0]["meta"]["count"] == 1

        id = resp_insert[0]["data"][0]

        query_get = f"""
            collection|test|: get[
                        {to_str(id)},
                    ]
            """
        resp_get = conn.send_query(query_get)
        assert resp_get[0]["data"][id] == native_resp
        assert resp_get[0]["data"][id] == conv_resp

    def test_insert_string_with_special_symbols(self, conn):
        text = "smth |one| two,: three !@#$%^&*()_//\\"
        query_insert = f"""
                collection|test|: insert[
                            {to_str(String(text))},
                        ]
                """
        conn.send_query(query_insert)

        query_2 = """
            collection|test|: find[];
            """
        resp = conn.send_query(query_2)
        for i in resp[0].data.values():
            assert i == text

    def test_insert_link(self, conn):
        query_insert = """
        collection|test|: insert[
                    n|101|,
                ]
        """
        resp_insert = conn.send_query(query_insert)
        link = resp_insert[0]["data"][0]

        query_insert = f"""
                collection|test|: insert[
                            {to_str(link)},
                        ]
                """
        resp_insert = conn.send_query(query_insert)

        assert type(resp_insert[0]["data"][0]) == Link
        assert type(resp_insert[0]["data"][0].value) == UUID
        assert resp_insert[0]["meta"]["count"] == 1

        id = resp_insert[0]["data"][0]

        query_get = f"""
            collection|test|: get[
                        {to_str(id)},
                    ]
            """
        resp_get = conn.send_query(query_get)
        assert resp_get[0]["data"][id] == 101
        assert resp_get[0]["data"][id] == Number(101)


class TestInsertContainers:
    @pytest.mark.parametrize(
        "query",
        [
            """
            collection|test|: insert[
                        v[
                            n|1|,
                            s|2|,
                            b|true|,
                            v[
                                n|101|,
                                n|102|,
                                n|103|,
                            ]
                        ],
                    ]
            """,
            QuerySet(
                Insert([1, "2", True, [101, 102, 103]]), collection="test"
            ),
        ],
    )
    def test_insert_vector(self, conn, query):
        resp_insert = conn.send_query(query)
        assert type(resp_insert[0]["data"][0]) == Link
        assert type(resp_insert[0]["data"][0].value) == UUID
        assert resp_insert[0]["meta"]["count"] == 1

        id = resp_insert[0]["data"][0]

        query_get = f"""
                    collection|test|: get[
                                {to_str(id)},
                            ]
                    """
        resp_get = conn.send_query(query_get)
        assert resp_get[0]["data"][id] == [1, "2", True, [101, 102, 103]]
        assert resp_get[0]["data"][id] == Vector(1, "2", True, [101, 102, 103])

    @pytest.mark.parametrize(
        "query",
        [
            """
            collection|test|: insert[
                        m{
                            s|foo|:s|bar|,
                            s|vec|:v[
                                n|101|,
                                n|102|,
                                n|103|,
                            ]
                        },
                    ]
            """,
            QuerySet(
                Insert({"foo": "bar", "vec": [101, 102, 103]}),
                collection="test",
            ),
        ],
    )
    def test_insert_map(self, conn, query):
        resp_insert = conn.send_query(query)
        assert type(resp_insert[0]["data"][0]) == Link
        assert type(resp_insert[0]["data"][0].value) == UUID
        assert resp_insert[0]["meta"]["count"] == 1

        id = resp_insert[0]["data"][0]

        query_get = f"""
                    collection|test|: get[
                                {to_str(id)},
                            ]
                    """
        resp_get = conn.send_query(query_get)
        assert resp_get[0]["data"][id] == {
            "foo": "bar",
            "vec": [101, 102, 103],
        }
        assert resp_get[0]["data"][id] == Map(
            {"foo": "bar", "vec": [101, 102, 103]}
        )


class TestInsertMany:
    @pytest.mark.parametrize(
        "query",
        [
            """
            collection|test|: insert[
                        n|1|,
                        s|2|,
                        b|true|,
                        v[
                            n|101|,
                            n|102|,
                            n|103|,
                        ]
                    ]
            """,
            QuerySet(Insert(1, "2", True, [101, 102, 103]), collection="test"),
        ],
    )
    def test_insert_vector(self, conn, query):
        resp_insert = conn.send_query(query)
        assert type(resp_insert[0]["data"][0]) == Link
        assert type(resp_insert[0]["data"][0].value) == UUID
        assert resp_insert[0]["meta"]["count"] == 4

        ids = ",".join([to_str(i) for i in resp_insert[0]["data"]])

        query_get = f"""
                    collection|test|: get[
                                {ids},
                            ]
                    """
        resp_get = conn.send_query(query_get)
        for i in resp_get[0]["data"].values():
            assert i in [1, "2", True, [101, 102, 103]]
