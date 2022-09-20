from annadb.dump import to_str


class TestDelete:
    def test_delete_collection(self, conn, primitives):
        resp_delete = conn.send_query(
            """
            collection|test|:delete
            """
        )
        assert resp_delete[0]["meta"]["count"] == 0
        resp = conn.send_query(
            """
            collection|test|:find[]
            """
        )
        assert len(resp[0]["data"]) == 0

    def test_delete_all(self, conn, primitives):
        resp_delete = conn.send_query(
            """
            collection|test|:q[
                find[],
                delete
            ]
            """
        )
        assert resp_delete[0]["meta"]["count"] == primitives["meta"]["count"]
        resp = conn.send_query(
            """
            collection|test|:find[]
            """
        )
        assert len(resp[0]["data"]) == 0

    def test_delete_many(self, conn, primitives):
        resp_delete = conn.send_query(
            """
            collection|test|:q[
                find[
                    gt{root:n|2|}
                ],
                delete
            ]
            """
        )
        assert resp_delete[0]["meta"]["count"] == 3
        resp = conn.send_query(
            """
            collection|test|:find[]
            """
        )
        assert len(resp[0]["data"]) == primitives["meta"]["count"] - 3

    def test_delete_one(self, conn, primitives):

        resp_delete = conn.send_query(
            f"""
            collection|test|:q[
                get[
                    {to_str(primitives["data"][0])}
                ],
                delete
            ]
            """
        )
        assert resp_delete[0]["meta"]["count"] == 1
        resp = conn.send_query(
            """
            collection|test|:find[]
            """
        )
        assert len(resp[0]["data"]) == primitives["meta"]["count"] - 1
