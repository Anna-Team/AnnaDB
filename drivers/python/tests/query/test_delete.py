from pprint import pprint

from annadb.data_types.constants import root


class TestDelete:
    def test_delete_collection(self, collection, primitives):
        resp_delete = collection.delete().run()
        assert resp_delete.meta.count == 0
        resp = collection.all().run()
        assert len(resp.data) == 0
        assert resp.meta.count == 0

    def test_delete_all(self, collection, primitives):
        resp_delete = collection.all().delete().run()
        assert resp_delete.meta.count == primitives.meta.count
        resp = collection.all().run()
        assert len(resp.data) == 0
        assert resp.meta.count == 0

    def test_delete_many(self, collection, primitives):
        resp_delete = collection.find(root > 2).delete().run()
        assert resp_delete.meta.count == 3
        resp = collection.all().run()
        assert len(resp.data) == primitives.meta.count - 3

    def test_delete_one(self, collection, primitives):
        resp_delete = collection.get(primitives.data[0]).delete().run()
        assert resp_delete.meta.count == 1
        resp = collection.all().run()
        pprint(resp.data.items())
        assert len(resp.data) == primitives.meta.count - 1
