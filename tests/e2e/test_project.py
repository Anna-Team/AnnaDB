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
        print(resp[0])
        for k, v in resp[0]["data"].items():
            print(v._value)
            # assert list(v.keys()) == ["name"]
