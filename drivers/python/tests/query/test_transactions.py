from annadb.data_types.constants import root
from annadb.query.update.operators import Set


class TestTransactions:
    def test_find(self, conn, collection, objects):
        t = conn.new_transaction()
        t.push(collection.find(root.num > 4))
        t.push(collection.find(root.is_even == True))
        resp = t.run()
        assert len(resp[0].data) == 5
        assert len(resp[1].data) == 5
        for k, v in resp[0].data.items():
            assert v["num"] > 1
        for k, v in resp[1].data.items():
            assert v["is_even"] == True

    def test_update(self, conn, objects, collection):
        t = conn.new_transaction()
        t.push(collection.all().update(Set({root.blink2.a: 100})))
        t.push(collection.all().update(Set({root.blink2.e: 1000})))
        t.run()
        resp = collection.all().run()
        for k, v in resp.data.items():
            assert v["blink2"]["a"] == 100
            assert v["blink2"]["e"] == 1000

    def test_update_and_find(self, conn, objects, collection):
        t = conn.new_transaction()
        t.push(collection.all().update(Set({root.blink2.a: 100})))
        t.push(collection.all().update(Set({root.blink2.e: 1000})))
        t.push(collection.all())
        resp = t.run()
        for k, v in resp[2].data.items():
            assert v["blink2"]["a"] == 100
            assert v["blink2"]["e"] == 1000

    def test_insert(self, conn, collection):
        t = conn.new_transaction()
        t.push(collection.insert({"vec": [101, 102, 103], "foo": "bar"}))
        t.push(collection.insert({"vec": [101, 102, 103], "foo": "bar"}))
        t.run()
        resp = collection.all().run()
        assert len(resp.data) == 2
        for k, v in resp.data.items():
            assert v["vec"][0] == 101
            assert v["foo"] == "bar"

    def test_insert_find(self, conn, collection):
        t = conn.new_transaction()
        t.push(collection.insert({"vec": [101, 102, 103], "foo": "bar"}))
        t.push(collection.insert({"vec": [101, 102, 103], "foo": "bar"}))
        t.push(collection.all())
        resp = t.run()
        assert len(resp[2].data) == 2
        for k, v in resp[2].data.items():
            assert v["vec"][0] == 101
            assert v["foo"] == "bar"
