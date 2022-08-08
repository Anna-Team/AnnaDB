from annadb.dump import to_str


class TestRecursion:
    def test_recursion(self, conn):
        query_1 = """
        collection|test|: insert[
                              s|foo|,
                          ]
        """
        resp_1 = conn.send_query(query_1)
        id_1 = resp_1[0]["data"][0]

        query_2 = f"""
        collection|test|: insert[
                    {to_str(id_1)},
                ]
        """
        resp_2 = conn.send_query(query_2)
        id_2 = resp_2[0]["data"][0]

        query_3 = f"""
        collection|test|: q[
            get[{to_str(id_1)}],
            update[
                set{{root: {to_str(id_2)}}}
            ]
        ]
        """
        conn.send_query(query_3)

        result_query = """
        collection|test|: find[]
        """
        res = conn.send_query(result_query)
        assert res == "Fetch recursion error"
