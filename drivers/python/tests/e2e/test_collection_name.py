class TestCollectionName:
    def test_wrong_name(self, conn):
        query_insert = """
                        collection|_test|: insert[
                                    s|foo|,
                                ]
                        """
        res = conn.send_query(query_insert)

        assert res == "Invalid collection name: _test"
