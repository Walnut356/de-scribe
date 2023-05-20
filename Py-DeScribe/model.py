from abc import ABC
from dataclasses import dataclass
from datetime import datetime

from .utils import IonType


class IonDType(ABC):
    length: int
    raw: list[int]
    offset: int


@dataclass
class Null(IonDType):
    pass


@dataclass
class RawInt(IonDType):
    value: int


@dataclass
class RawUInt(IonDType):
    value: int


@dataclass
class VarUInt(IonDType):
    value: int


@dataclass
class VarInt(IonDType):
    value: int


@dataclass
class Boolean(IonDType):
    value: bool


@dataclass
class UInt(IonDType):
    value: int


@dataclass
class Int(IonDType):
    value: int


@dataclass
class Float(IonDType):
    value: float


@dataclass
class Decimal(IonDType):
    value: int


@dataclass
class TimeStamp(IonDType):
    value: datetime


@dataclass
class Symbol(IonDType):
    value: int


@dataclass
class String(IonDType):
    value: str


@dataclass
class Clob(IonDType):
    pass


@dataclass
class Blob(IonDType):
    pass


@dataclass
class List(IonDType):
    elements: list[IonDType]


@dataclass
class Sexp(IonDType):
    elements: list[IonDType]


@dataclass
class Struct(IonDType):
    fields: dict


@dataclass
class Annotations(IonDType):
    annot_length: int
    annot: int
    value: bytearray


TYPE_INIT_MAP = {
    IonType.NULL: Null,
    IonType.BOOL: Boolean,
    IonType.POS_INT: UInt,
    IonType.NEG_INT: UInt,
    IonType.FLOAT: Float,
    IonType.DECIMAL: Decimal,
    IonType.TIMESTAMP: TimeStamp,
    IonType.SYMBOL: Symbol,
    IonType.STRING: String,
    IonType.CLOB: Clob,
    IonType.BLOB: Blob,
    IonType.LIST: List,
    IonType.SEXP: Sexp,
    IonType.STRUCT: Struct,
    IonType.ANNOT: Annotations,
    IonType.RESERVED: None,
}
