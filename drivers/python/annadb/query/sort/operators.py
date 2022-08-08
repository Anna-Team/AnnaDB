from annadb.constants import ASC_OPERATOR, DESC_OPERATOR
from annadb.data_types.modifier import ModifierBase


class Asc(ModifierBase):
    prefix = ASC_OPERATOR


class Desc(ModifierBase):
    prefix = DESC_OPERATOR
