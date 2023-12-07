## `⌽` MicroDB
Serialized database for Microcontrollers with memory efficiency in mind.
```
CURRENT PERFORMANCE:
Iterates over an entire 7MB database file containing 100K entries
while only utilizing 1KB on the heap in 1700ms (17μs per entry).
```

___
### `➢` Structure (Not Finalized)
#### `⤷` 8-bytes aligned database structure
```
EOE: End-Of-Entry
An arbitrary series of bytes to indicate the end of an entry (needs more research).
```


```
|   00   |   01   |   02   |   03   |   04   |   05   |   06   |   07   |
|--------|--------|--------|--------|--------|--------|--------|--------|
|                            > FIRST CHUNK <                            |
|  0x01  |  0x00  |  0x00  |  0x00  |  0x00  |  0x00  |  0x00  |  0x00  | -> Entry UID
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  0xFF  |  0xFE  |  0xFD  |  0xFC  |  0xFB  |  0xFA  |  0xF9  |  0xF9  | -> EOE Block
|                           > SECOND CHUNK <                            |
|  0x02  |  0x00  |  0x00  |  0x00  |  0x00  |  0x00  |  0x00  |  0x00  | -> Entry UID
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  0xFF  |  0xFE  |  0xFD  |  0xFC  |  0xFB  |  0xFA  |  0xF9  |  0xF9  | -> EOE Block
```