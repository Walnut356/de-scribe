import mmap
from enum import IntEnum


class IonType(IntEnum):
    NULL = 0x0
    BOOL = 0x1
    POS_INT = 0x2
    NEG_INT = 0x3
    FLOAT = 0x4
    DECIMAL = 0x5
    TIMESTAMP = 0x6
    SYMBOL = 0x7
    STRING = 0x8
    CLOB = 0x9
    BLOB = 0xA
    LIST = 0xB
    SEXP = 0xC
    STRUCT = 0xD
    ANNOT = 0xE
    RESERVED = 0xF


def find_start(file: mmap.mmap):
    for i in range(len(file)):
        if file[i:i + 5] == b'\xe0\x01\x00\xea\xee':
            yield i + 4


def dtype_from_bytes(file: mmap.mmap) -> tuple:
    val = int.from_bytes(file.read(1), "big")
    dtype = IonType((val & 0xF0) >> 4)
    length = val & 0x0F
    match length:
        case 14:
            length = decode_varuint(file)
        case 15:
            length = 0
        case _:
            pass

    return dtype, length


def decode_varuint(file: mmap.mmap) -> int:
    valid = False
    total = 0x0
    while True:
        val = int.from_bytes(file.read(1), "big")
        if val >= 0x80:
            val &= 0x7F
            total = total >> 1
            total |= val
            valid = True
            break
        else:
            total |= val
            total = total << 8
    if not valid:
        raise ValueError("VarUInt terminator value not found")
    return total
