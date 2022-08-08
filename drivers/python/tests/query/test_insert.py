from uuid import UUID

from annadb.data_types.primitive import Link


class TestInsert:
    def test_primitive_one(self, collection):
        resp_insert = collection.insert("foo").run()
        assert type(resp_insert.data[0]) == Link
        assert type(resp_insert.data[0].value) == UUID
        assert resp_insert.meta.count == 1

        id = resp_insert.data[0]
        resp_get = collection.get(id).run()
        assert resp_get.data[id] == "foo"

    def test_primitive_many(self, collection):
        resp_insert = collection.insert("foo", 1, True, None).run()
        assert resp_insert.meta.count == 4

        resp_get = collection.find().run()
        for item in resp_get.data.values():
            assert item in ["foo", 1, True, None]

    def test_insert_vector(self, collection):
        resp_insert = collection.insert(["foo", 1, True, None]).run()
        assert type(resp_insert.data[0]) == Link
        assert type(resp_insert.data[0].value) == UUID
        assert resp_insert.meta.count == 1

        id = resp_insert.data[0]
        resp_get = collection.get(id).run()
        assert resp_get.data[id] == ["foo", 1, True, None]

    def test_insert_map(self, collection):
        resp_insert = collection.insert({"foo": 1, "bar": None}).run()
        assert type(resp_insert.data[0]) == Link
        assert type(resp_insert.data[0].value) == UUID
        assert resp_insert.meta.count == 1

        id = resp_insert.data[0]
        resp_get = collection.get(id).run()
        assert resp_get.data[id] == {"foo": 1, "bar": None}
