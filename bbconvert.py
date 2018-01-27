""" Converts block buffer data from in-memory representation
(blocks of 13*16 bytes at 0x500-0x5cf and 0x5d0-0x69f)
into a 2-D map usable in the blockbuf! macro.
"""
import sys

buf = ''
for s in sys.stdin:
    buf += s.strip()

rows = [''.join([buf[i:i+32] for i in range(y*16*2, len(buf), 16*2*13)]) for y in range(13)]
hexrows = ['[' + ','.join([('  0 ' if row[i:i+2] == '00' else '0x' + row[i:i+2]) for i in range(0,len(row),2)]) + ']' for row in rows]

print('[' + ',\n '.join(hexrows) + ']')
