from annadb.data_types.constants import root


class TestOffset:
    def test_after_sort(self, collection, primitives):
        resp = collection.all().sort(+root).offset(10).run()
        assert len(resp.data) == 5
        assert list(resp.data.values()) == [1000, 2000, False, True, None]

    def test_after_find(self, collection, primitives, objects):
        resp = collection.all().offset(10).run()
        assert len(resp.data) == 15

        resp = collection.all().offset(-10).run()
        assert len(resp.data) == 25

        resp = collection.all().offset(3.1).run()
        assert len(resp.data) == 22

        resp = collection.all().offset(45).run()
        assert len(resp.data) == 0
