import struct
import random

with open('./large_binary_file', 'rb') as infile, open('./large_binary_file2', 'wb') as of:
    while True:
        d = infile.read(4)
        if not d:
            break
        if random.randint(0,1) == 1:
            le = struct.unpack('<I', d)
            be = struct.pack('>I', *le)
            of.write(be)
        else:
            of.write(d)
