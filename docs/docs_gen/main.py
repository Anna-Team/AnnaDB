from annadb.connection import Connection
from delete import build_delete
from find import build_find
from get import build_get
from insert import build_insert
from introduction import build_intro
from limit import build_limit
from offset import build_offset
from sort import build_sort
from update import build_update

connection = Connection.from_connection_string("annadb://localhost:10001")


def clean_collection():
    collection = connection["test"]
    collection.delete().run()


build_intro()

clean_collection()
build_insert(connection)

clean_collection()
build_get(connection)

clean_collection()
build_find(connection)

clean_collection()
build_sort(connection)

clean_collection()
build_limit(connection)

clean_collection()
build_offset(connection)

clean_collection()
build_update(connection)

clean_collection()
build_delete(connection)
