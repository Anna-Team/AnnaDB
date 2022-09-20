class TestOffset:
    def test_after_sort(self, conn, primitives):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                sort[asc(root)],
                offset(n|10|)
            ]
            """
        )
        assert len(resp[0]["data"]) == 5
        assert list(resp[0]["data"]._value.values()) == [
            1000,
            2000,
            False,
            True,
            None,
        ]

    def test_after_find(self, conn, primitives, objects):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                offset(n|10|)
            ]
            """
        )
        assert len(resp[0]["data"]) == 15

        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                offset(n|-10|)
            ]
            """
        )
        assert len(resp[0]["data"]) == 25

        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                offset(n|3.1|)
            ]
            """
        )
        assert len(resp[0]["data"]) == 22

        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                offset(n|45|)
            ]
            """
        )
        assert len(resp[0]["data"]) == 0
