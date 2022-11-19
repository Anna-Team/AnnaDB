class TestProject:
    def test_keep(self, conn, objects):
        resp = conn.send_query(
            """
            collection|test|:q[
                find[],
                project{
                    s|name|:keep
                }
            ]
            """
        )
        for k, v in resp[0]["data"].items():
            assert list(v.keys()) == ["name"]
