from annadb.constants import (
    EQ_OPERATOR,
    NEQ_OPERATOR,
    GT_OPERATOR,
    GTE_OPERATOR,
    LT_OPERATOR,
    LTE_OPERATOR,
    AND_OPERATOR,
    OR_OPERATOR,
    NOT_OPERATOR,
)
from annadb.data_types.map import MapUnique
from annadb.data_types.modifier import ModifierBase
from annadb.data_types.vector import VectorBase


class BaseFindOperator:
    def __and__(self, other):
        return And(self, other)

    def __or__(self, other):
        return Or(self, other)


class Eq(BaseFindOperator, MapUnique):
    prefix = EQ_OPERATOR


class Neq(BaseFindOperator, MapUnique):
    prefix = NEQ_OPERATOR


class Gt(BaseFindOperator, MapUnique):
    prefix = GT_OPERATOR


class Gte(BaseFindOperator, MapUnique):
    prefix = GTE_OPERATOR


class Lt(BaseFindOperator, MapUnique):
    prefix = LT_OPERATOR


class Lte(BaseFindOperator, MapUnique):
    prefix = LTE_OPERATOR


class And(BaseFindOperator, VectorBase):
    prefix = AND_OPERATOR


class Or(BaseFindOperator, VectorBase):
    prefix = OR_OPERATOR


class Not(BaseFindOperator, ModifierBase):
    prefix = NOT_OPERATOR
