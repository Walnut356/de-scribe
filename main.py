import amazon.ion.reader_managed as ion_m
import amazon.ion.reader_binary as ion_b
import amazon.ion.simpleion as ion
from amazon.ion.reader import NEXT_EVENT, BufferQueue, blocking_reader
from pathlib import Path
from io import BytesIO
file = Path(R"blank with dots/Center dot")

with open(file, "rb") as f:
    f.seek(25071)
    thing = BytesIO(f.read(529))
    eef = blocking_reader(ion_b.binary_reader(), thing)



    while True:
        eef.send(NEXT_EVENT)
        try:
            event = eef.gi_frame.f_locals['ion_event'].event_type.name
        except:
            event = None
        try:
            type = eef.gi_frame.f_locals['ion_event'].ion_type.name
        except:
            type = None
        try:
            name = eef.gi_frame.f_locals['ion_event'].field_name
        except:
            name = None
        try: value = eef.gi_frame.f_locals['ion_event'].value
        except: value = None
        try:
            print(f"Event: {event}, Type: {type}, Name: {name}\n Value: {value}\n")
        except (AttributeError, TypeError):
            continue