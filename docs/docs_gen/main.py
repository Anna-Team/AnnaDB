from annadb.connection import Connection
from delete import build_delete
from find import build_find
from get import build_get
from insert import build_insert
from introduction import build_intro
from limit import build_limit
from native_tutorial import build_native_tutorial
from offset import build_offset
from sort import build_sort
from update import build_update

connection = Connection.from_connection_string("annadb://localhost:10001")


def clean_collections():
    collections = [connection["test"], connection["products"], connection["categories"]]
    for collection in collections:
        collection.delete().run()


build_intro()

clean_collections()
build_insert(connection)

clean_collections()
build_get(connection)

clean_collections()
build_find(connection)

clean_collections()
build_sort(connection)

clean_collections()
build_limit(connection)

clean_collections()
build_offset(connection)

clean_collections()
build_update(connection)

clean_collections()
build_delete(connection)

clean_collections()
build_native_tutorial(connection)
