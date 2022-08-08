from uuid import UUID

from annadb.constants import (
    STRING,
    NUMBER,
    BOOL,
    NULL,
    DELETED,
    COLLECTION_NAME,
    UTS_PREFIX,
)
from annadb.data_types.tyson import TysonItem


class PrimitiveBase(TysonItem):
    prefix_color = "yellow"
    value_color = "cyan"
    instance_type = "primitive"

    def __init__(self, value):
        self.value = value

    @property
    def pretty_value(self):
        return f"[{self.value_color}]{self.value}[/{self.value_color}]"

    @property
    def html_value(self):
        return f'<span class="value_{self.instance_type}">{self.value}</span>'

    def pretty(self, console):
        if self.value:
            console.add_text(f"{self.pretty_prefix}|{self.pretty_value}|")
        else:
            console.add_text(f"{self.pretty_prefix}")

    def to_html(self, console):
        if self.value:
            console.add_text(f"{self.html_prefix}|{self.html_value}|")
        else:
            console.add_text(f"{self.html_prefix}")

    def __hash__(self):
        return hash(self.prefix) + hash(self.value)

    def __eq__(self, other):
        if isinstance(other, PrimitiveBase):
            return self.prefix == other.prefix and self.value == other.value
        return False


class String(str, PrimitiveBase):
    value_color = "bright_green"
    prefix = STRING
    instance_type = "string"

    @classmethod
    def from_serialized(cls, value: str):
        value = value.replace("\\\\", "\\").replace("\\|", "|")
        return cls(value)


class Number(float, PrimitiveBase):
    value_color = "bright_blue"
    prefix = NUMBER
    instance_type = "number"


class UTS(int, PrimitiveBase):
    value_color = "bright_blue"
    prefix = UTS_PREFIX
    instance_type = "number"


class Bool(PrimitiveBase):
    value_color = "bright_red"
    prefix = BOOL
    instance_type = "bool"

    def __init__(self, value):
        if not isinstance(value, bool):
            if value == "true":
                value = True
            elif value == "false":
                value = False
            else:
                raise TypeError("Not bool")
        super(Bool, self).__init__(value)

    def __eq__(self, other):
        if isinstance(other, bool):
            return self.value is other
        return super(Bool, self).__eq__(other)


class Null(PrimitiveBase):
    prefix = NULL
    instance_type = "null"

    def __init__(self, _=None):
        super(Null, self).__init__(None)

    def __eq__(self, other):
        if other is None:
            return True
        if isinstance(other, PrimitiveBase):
            return self.prefix == other.prefix
        return False


class Deleted(PrimitiveBase):
    prefix = DELETED

    def __init__(self, _=None):
        super(Deleted, self).__init__(None)


class CollectionName(PrimitiveBase):
    prefix = COLLECTION_NAME


class Link(PrimitiveBase):
    prefix = ""
    instance_type = "link"

    def __init__(self, value, prefix):
        self.prefix = prefix
        value = UUID(value)
        super(Link, self).__init__(value)
