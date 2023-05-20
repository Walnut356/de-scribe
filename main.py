import os
from pathlib import Path

from _pydescribe.utils import *

source = Path("partial erase/pre_erase")

data = []

with mmap.mmap(os.open(source, os.O_RDONLY), 0, access=mmap.ACCESS_READ) as file:
    base_offsets = find_start(file)
    for base_offset in base_offsets:
        file.seek(base_offset)
        if base_offset == 23203:
            temp = dtype_from_bytes(file)

            print(temp)
            break
