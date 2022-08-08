from annadb.data_types.constants import root
from annadb.query.update.operators import Inc, Set


class TestUpdateInc:
    def test_root(self, collection, primitives):
        collection.find(root == 1).update(Inc({root: 2})).run()
        resp = collection.get(primitives.data[0]).run()
        for k, v in resp.data.items():
            assert k == primitives.data[0]
            assert v == 3

    def test_object(self, collection, objects):
        collection.find(root.is_even == True).update(
            Inc({root.num: 100})
        ).run()
        resp = collection.find(root.is_even == True).run()
        for k, v in resp.data.items():
            assert v["num"] >= 100


class TestUpdateSet:
    def test_root(self, collection, primitives):
        collection.find(root == 1).update(Set({root: 100})).run()
        resp = collection.get(primitives.data[0]).run()
        for k, v in resp.data.items():
            assert k == primitives.data[0]
            assert v == 100

    def test_object(self, collection, objects):
        collection.find(root.is_even == True).update(
            Set({root.num: 100})
        ).run()
        resp = collection.find(root.is_even == True).run()
        for k, v in resp.data.items():
            assert v["num"] == 100

    def test_rewrite_object(self, collection):
        collection.insert({"foo": "bar"}).run()
        collection.find(root.foo == "bar").update(
            Set({root: {"a": "b"}})
        ).run()
        resp = collection.find().run()
        for k, v in resp.data.items():
            print(v.items())
            assert v["a"] == "b"


class TestUpdateOther:
    def test_field_not_exists_set(self, collection, objects):
        collection.all().update(Set({root.blink2.a: 100})).run()
        collection.all().update(Set({root.blink2.e: 1000})).run()
        resp = collection.all().run()
        for k, v in resp.data.items():
            assert v["blink2"]["a"] == 100
            assert v["blink2"]["e"] == 1000

    def test_double_update(self, conn, objects):
        conn.send_query(
            """
            collection|test|:q[
                find[
                    eq{value|is_even|:b|true|}
                ],
                update[
                    set{value|blink|:n|100|},
                    set{value|num|:n|200|},
                ]
            ];"""
        )
        resp = conn.send_query(
            """collection|test|:find[
                    eq{value|is_even|:b|true|}
                ]
            """
        )
        for k, v in resp[0]["data"].items():
            assert v["blink"] == 100
            assert v["num"] == 200

    def test_by_linked_field_primitive(self, collection):
        resp = collection.insert_one("bar").run()
        id = resp.data

        collection.insert_one({"foo": id}).run()

        resp = (
            collection.find(root.foo == "bar")
            .update(Set({root.foo: "baz"}))
            .run()
        )
        assert resp.meta.count == 1

        resp = collection.get_one(id).run()
        assert resp.data == "bar"

    def test_by_linked_field_container(self, collection):
        resp = collection.insert_one({"bar": "baz"}).run()
        id = resp.data
        collection.insert_one({"foo": id}).run()
        resp = (
            collection.find(root.foo.bar == "baz")
            .update(Set({root.foo.bar: "bazzz"}))
            .run()
        )
        assert resp.meta.count == 1

        resp = collection.get_one(id).run()
        assert resp.data["bar"] == "bazzz"
