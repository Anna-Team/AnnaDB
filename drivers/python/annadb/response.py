from annadb.constants import (
    RESPONSE_IDS,
    OK_RESPONSE,
    GET_META,
    INSERT_META,
    FIND_META,
    UPDATE_META,
    DELETE_META,
    QUERY_RESPONSE,
    RESPONSE_OBJECTS,
    RESPONSE_PRIMITIVE,
    ERROR_RESPONSE,
)
from annadb.data_types.map import MapUnique
from annadb.data_types.primitive import PrimitiveBase
from annadb.data_types.vector import VectorBase


class IDs(VectorBase):
    prefix = RESPONSE_IDS


class OkResponse(VectorBase):
    prefix = OK_RESPONSE


class GetMeta(MapUnique):
    prefix = GET_META

    @property
    def count(self):
        return self["count"]


class InsertMeta(MapUnique):
    prefix = INSERT_META

    @property
    def count(self):
        return self["count"]


class FindMeta(MapUnique):
    prefix = FIND_META

    @property
    def count(self):
        return self["count"]


class UpdateMeta(MapUnique):
    prefix = UPDATE_META

    @property
    def count(self):
        return self["count"]


class DeleteMeta(MapUnique):
    prefix = DELETE_META

    @property
    def count(self):
        return self["count"]


class QueryResponse(MapUnique):
    prefix = QUERY_RESPONSE

    @property
    def data(self):
        return self["data"]

    @property
    def meta(self):
        return self["meta"]


class Objects(MapUnique):
    prefix = RESPONSE_OBJECTS


class ResponsePrimitive(PrimitiveBase):
    prefix = RESPONSE_PRIMITIVE


class ErrorResponse(str, PrimitiveBase):
    prefix = ERROR_RESPONSE
