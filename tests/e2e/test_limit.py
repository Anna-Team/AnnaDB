class TestLimit:
    def test_after_sort(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                sort[asc(root)],
                limit(n|10|)
            ]
            """
        )
        assert len(resp[0]["data"]) == 10
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
        ]

    def test_after_find(self, conn, primitives, objects):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                limit(n|10|)
            ]
            """
        )
        assert len(resp[0]["data"]) == 10

        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                limit(n|-10|)
            ]
            """
        )
        assert len(resp[0]["data"]) == 0

        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                limit(n|3.1|)
            ]
            """
        )
        assert len(resp[0]["data"]) == 3

        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                limit(n|45|)
            ]
            """
        )
        assert len(resp[0]["data"]) == 25
