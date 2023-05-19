import enum
import io
import struct
#import bitstring
from dataclasses import dataclass

class ObjectCode(enum.Enum):
    NULL = 0
    BOOL = 1
    INT_P = 2
    INT_N = 3
    FLOAT = 4
    DECIMAL = 5
    TIMESTAMP = 6
    SYMBOL = 7
    STRING = 8
    CLOB = 9
    BLOB = 10
    LIST = 11
    S_EXPR = 12
    STRUCT = 13
    ANNOT = 14
    RESERVED = 15

@dataclass
class IonObject():
    type: ObjectCode
    length: int

def get_format(byte_code: int):
    return IonObject(ObjectCode(byte_code >> 4), byte_code & 0x0F)

# def decode_varuint(stream: io.BytesIO):
#     temp = stream.read(1)
#     while temp == 0:
#         if temp & 0x80 == 0:
