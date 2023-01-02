import pytest

from annadb.connection import Connection
from annadb.data_types.primitive import UTS


@pytest.fixture(autouse=True)
def rm_db(conn):
    conn.send_query(
        """
    collection|test|:delete;
    collection|test2|:delete;
    collection|test_big|:delete;
    collection|_internal|:delete;
    """
    )


@pytest.fixture
def conn():
    return Connection.from_connection_string("annadb://localhost:10001")


@pytest.fixture
def collection(conn):
    return conn["test"]


@pytest.fixture
def primitives(collection):
    resp = collection.insert(
        1,
        2,
        3,
        4,
        5,
        UTS(1000),
        UTS(2000),
        "test1",
        "test2",
        "test3",
        "test4",
        "test5",
        True,
        False,
        None,
    ).run()
    return resp


@pytest.fixture
def objects(collection):
    objs = [
        {
            "name": f"test_{i}",
            "num": i,
            "is_even": i % 2 == 0,
            "smth": "TEST",
            "blink": "test" if i % 2 == 0 else {"smth": i},
            "blink2": {"a": 100, "c": "d"} if i % 2 == 0 else {},
            "ts": UTS(12345),
            "d": {"smth": i}
        }
        for i in range(10)
    ]
    resp = collection.insert(*objs).run()
    return resp
