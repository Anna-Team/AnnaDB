from typing import Optional

from annadb.constants import PATH_TO_VALUE, ROOT
from annadb.data_types.primitive import PrimitiveBase
from annadb.query.find.operators import Eq, Gt, Gte, Lt, Lte, Neq
from annadb.query.sort.operators import Asc, Desc


class Path(PrimitiveBase):
    def __init__(self, value: Optional[str] = None):
        if value is None or value == "":
            self.prefix = ROOT
        else:
            self.prefix = PATH_TO_VALUE
        super(Path, self).__init__(value)

    def __getitem__(self, item):
        """
        Get sub field

        :param item: name of the subfield
        :return: Path
        """
        if self.value is not None:
            return Path(f"{self.value}.{item}")
        else:
            return Path(f"{item}")

    def __getattr__(self, item):
        """
        Get sub field

        :param item: name of the subfield
        :return: Path
        """
        if self.value is not None:
            return Path(f"{self.value}.{item}")
        else:
            return Path(f"{item}")

    def __eq__(self, other):
        return Eq({self: other})

    def __gt__(self, other):
        return Gt({self: other})

    def __ge__(self, other):
        return Gte({self: other})

    def __lt__(self, other):
        return Lt({self: other})

    def __le__(self, other):
        return Lte({self: other})

    def __ne__(self, other):
        return Neq({self: other})

    def __pos__(self):
        return Asc(self)

    def __neg__(self):
        return Desc(self)

    def __hash__(self):
        return hash("__root__")


root = Path()
