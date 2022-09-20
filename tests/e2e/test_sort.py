class TestSort:
    def test_root_asc(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                sort[asc(root)]
            ]
            """
        )
        assert len(resp[0]["data"]) == 15
        assert list(resp[0]["data"]._value.values()) == [
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

    def test_root_desc(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                sort[desc(root)]
            ]
            """
        )
        assert len(resp[0]["data"]) == 15
        assert (
            list(resp[0]["data"]._value.values())
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

    def test_asc_desc(self, conn, primitives, objects):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                sort[desc(value|is_even|), asc(value|blink.smth|), asc(value|num|), desc(root)]
            ]
            """
        )
        assert len(resp[0]["data"]) == 25
        # TODO add assert for results
