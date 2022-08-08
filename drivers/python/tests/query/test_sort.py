from annadb.data_types.constants import root


class TestSort:
    def test_root_asc(self, collection, primitives):
        resp = collection.all().sort(+root).run()
        assert len(resp.data) == 15
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
            1000,
            2000,
            False,
            True,
            None,
        ]

    def test_root_desc(self, collection, primitives):
        resp = collection.all().sort(-root).run()
        assert len(resp.data) == 15
        assert (
            list(resp.data.values())
            == [
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
                1000,
                2000,
                False,
                True,
                None,
            ][::-1]
        )

    def test_asc_desc(self, collection, primitives, objects):
        resp = (
            collection.all()
            .sort(-root.is_even, +root.blink.smth, +root.num, -root)
            .run()
        )
        assert len(resp.data) == 25
        # TODO add assert for results
