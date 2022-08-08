from annadb.constants import (
    FIND_QUERY,
    GET_QUERY,
    INSERT_QUERY,
    SORT_QUERY,
    UPDATE_QUERY,
    OFFSET_QUERY,
    LIMIT_QUERY,
    DELETE_QUERY,
)
from annadb.data_types.modifier import ModifierBase
from annadb.data_types.primitive import PrimitiveBase
from annadb.data_types.vector import VectorBase


class Find(VectorBase):
    prefix = FIND_QUERY


class Get(VectorBase):
    prefix = GET_QUERY


class Insert(VectorBase):
    prefix = INSERT_QUERY


class Sort(VectorBase):
    prefix = SORT_QUERY


class Update(VectorBase):
    prefix = UPDATE_QUERY


class Offset(ModifierBase):
    prefix = OFFSET_QUERY


class Limit(ModifierBase):
    prefix = LIMIT_QUERY


class Delete(PrimitiveBase):
    prefix = DELETE_QUERY

    def __init__(self, _=None):
        super(Delete, self).__init__(None)
