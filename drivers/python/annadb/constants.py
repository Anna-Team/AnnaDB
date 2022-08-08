STRING: str = "s"
NUMBER: str = "n"
UTS_PREFIX: str = "uts"
BOOL: str = "b"
NULL: str = "null"
COLLECTION_NAME: str = "collection"
PATH_TO_VALUE: str = "value"
DELETED: str = "deleted"

STORAGE_MAP: str = "m"

STORAGE_VECTOR: str = "v"

# QUERIES
QUERY_SET: str = "q"
INSERT_QUERY: str = "insert"
FIND_QUERY: str = "find"
GET_QUERY: str = "get"
UPDATE_QUERY: str = "update"
DELETE_QUERY: str = "delete"
SORT_QUERY: str = "sort"
OFFSET_QUERY: str = "offset"
LIMIT_QUERY: str = "limit"

# FIND OPERATORS
EQ_OPERATOR: str = "eq"
NEQ_OPERATOR: str = "neq"
GT_OPERATOR: str = "gt"
GTE_OPERATOR: str = "gte"
LT_OPERATOR: str = "lt"
LTE_OPERATOR: str = "lte"

AND_OPERATOR: str = "and"
OR_OPERATOR: str = "or"

NOT_OPERATOR: str = "not"

# UPDATE OPERATORS
SET_OPERATOR: str = "set"
INC_OPERATOR: str = "inc"

# SORT OPERATORS
ASC_OPERATOR: str = "asc"
DESC_OPERATOR: str = "desc"

# RESPONSE
RESPONSE_OBJECTS: str = "objects"
RESPONSE_IDS: str = "ids"
RESPONSE: str = "data"
RESPONSE_PRIMITIVE: str = "result"
OK_RESPONSE: str = "ok"
ERROR_RESPONSE: str = "error"
QUERY_RESPONSE: str = "response"
INSERT_META: str = "insert_meta"
GET_META: str = "get_meta"
UPDATE_META: str = "update_meta"
FIND_META: str = "find_meta"
DELETE_META: str = "delete_meta"

# OTHER
ROOT: str = "root"
INTERNAL_COLLECTION_NAME: str = "_internal"
