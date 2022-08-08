from annadb.constants import (
    STRING,
    NUMBER,
    BOOL,
    NULL,
    COLLECTION_NAME,
    PATH_TO_VALUE,
    DELETED,
    STORAGE_MAP,
    STORAGE_VECTOR,
    INSERT_QUERY,
    FIND_QUERY,
    GET_QUERY,
    UPDATE_QUERY,
    EQ_OPERATOR,
    NEQ_OPERATOR,
    GT_OPERATOR,
    GTE_OPERATOR,
    LT_OPERATOR,
    LTE_OPERATOR,
    AND_OPERATOR,
    OR_OPERATOR,
    NOT_OPERATOR,
    SET_OPERATOR,
    INC_OPERATOR,
    RESPONSE_OBJECTS,
    RESPONSE_IDS,
    ROOT,
    DELETE_QUERY,
    RESPONSE_PRIMITIVE,
    OK_RESPONSE,
    ERROR_RESPONSE,
    INSERT_META,
    GET_META,
    QUERY_RESPONSE,
    UTS_PREFIX,
    FIND_META,
    UPDATE_META,
    DELETE_META,
    SORT_QUERY,
    ASC_OPERATOR,
    DESC_OPERATOR,
    QUERY_SET,
    LIMIT_QUERY,
    OFFSET_QUERY,
)
from annadb.data_types.map import Map
from annadb.data_types.primitive import (
    String,
    Number,
    Bool,
    Null,
    CollectionName,
    Deleted,
    Link,
    UTS,
)
from annadb.data_types.vector import Vector
from annadb.query.find.operators import Eq, Neq, Gt, Gte, Lt, Lte, And, Or, Not
from annadb.query.path import Path
from annadb.query.query_set import QuerySet
from annadb.query.sort.operators import Asc, Desc
from annadb.query.types import (
    Insert,
    Find,
    Get,
    Update,
    Delete,
    Sort,
    Limit,
    Offset,
)
from annadb.query.update.operators import Set, Inc
from annadb.response import (
    Objects,
    IDs,
    ResponsePrimitive,
    OkResponse,
    ErrorResponse,
    GetMeta,
    InsertMeta,
    FindMeta,
    UpdateMeta,
    DeleteMeta,
    QueryResponse,
)

types_map = {
    STRING: String,
    NUMBER: Number,
    UTS_PREFIX: UTS,
    BOOL: Bool,
    NULL: Null,
    COLLECTION_NAME: CollectionName,
    PATH_TO_VALUE: Path,
    DELETED: Deleted,
    STORAGE_MAP: Map,
    STORAGE_VECTOR: Vector,
    # QUERIES
    QUERY_SET: QuerySet,
    INSERT_QUERY: Insert,
    FIND_QUERY: Find,
    GET_QUERY: Get,
    UPDATE_QUERY: Update,
    DELETE_QUERY: Delete,
    SORT_QUERY: Sort,
    LIMIT_QUERY: Limit,
    OFFSET_QUERY: Offset,
    # FIND OPERATORS
    EQ_OPERATOR: Eq,
    NEQ_OPERATOR: Neq,
    GT_OPERATOR: Gt,
    GTE_OPERATOR: Gte,
    LT_OPERATOR: Lt,
    LTE_OPERATOR: Lte,
    AND_OPERATOR: And,
    OR_OPERATOR: Or,
    NOT_OPERATOR: Not,
    # UPDATE OPERATORS
    SET_OPERATOR: Set,
    INC_OPERATOR: Inc,
    # SORT OPERATORS
    ASC_OPERATOR: Asc,
    DESC_OPERATOR: Desc,
    # RESPONSE
    RESPONSE_OBJECTS: Objects,
    RESPONSE_IDS: IDs,
    RESPONSE_PRIMITIVE: ResponsePrimitive,
    OK_RESPONSE: OkResponse,
    ERROR_RESPONSE: ErrorResponse,
    GET_META: GetMeta,
    INSERT_META: InsertMeta,
    FIND_META: FindMeta,
    UPDATE_META: UpdateMeta,
    DELETE_META: DeleteMeta,
    QUERY_RESPONSE: QueryResponse,
    # OTHER
    ROOT: Path,
}


def build_primitive(item):
    cls = types_map.get(item.prefix, Link)
    if issubclass(cls, Link):
        return cls(item.value, prefix=item.prefix)
    elif issubclass(cls, String):
        return cls.from_serialized(item.value)
    else:
        return cls(item.value)


def factory(item):
    if i := item.primitive():
        return build_primitive(i)
    if i := item.modifier():
        cls = types_map[i.prefix]
        return cls(factory(i.get_value()))
    if i := item.map():
        obj = types_map[i.prefix]()
        for k, v in i.value:
            obj[build_primitive(k)] = factory(v)
        return obj
    if i := item.vector():
        cls = types_map[i.prefix]
        return cls(*[factory(j) for j in i.value])
