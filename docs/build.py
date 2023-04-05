from annadb.connection import Connection
from docs_gen.article_native_tutorial import build_first_article
from docs_gen.delete import build_delete
from docs_gen.find import build_find
from docs_gen.get import build_get
from docs_gen.insert import build_insert
from docs_gen.introduction import build_intro
from docs_gen.limit import build_limit
from docs_gen.native_tutorial import build_native_tutorial
from docs_gen.offset import build_offset
from docs_gen.project import build_project
from docs_gen.sort import build_sort
from docs_gen.update import build_update

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
build_project(connection)

clean_collections()
build_native_tutorial(connection)

clean_collections()
build_first_article(connection)
