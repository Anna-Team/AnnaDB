from annadb.data_types.constants import root


class TestLimit:
    def test_after_sort(self, collection, primitives):
        resp = collection.all().sort(+root).limit(10).run()
        assert len(resp.data) == 10
        assert list(resp.data.values()) == [
            "test1",
            "test2",
            "test3",
            "test4",
            "test5",
            1,
            2,
            3,
            4,
            5,
        ]

    def test_after_find(self, collection, primitives, objects):
        resp = collection.all().limit(10).run()
        assert len(resp.data) == 10
        resp = collection.all().limit(-10).run()
        assert len(resp.data) == 0

        resp = collection.all().limit(3.1).run()
        assert len(resp.data) == 3

        resp = collection.all().limit(45).run()
        assert len(resp.data) == 25
