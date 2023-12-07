## `⌽` MicroDB

### 8-bytes aligned database structure
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
|  0xFF  |  0xFE  |  0xFD  |  0xFC  |  0xFB  |  0xFA  |  0xF9  |  0xF9  | -> End-Of-Transmission Block
|                           > SECOND CHUNK <                            |
|  0x02  |  0x00  |  0x00  |  0x00  |  0x00  |  0x00  |  0x00  |  0x00  | -> Entry UID
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  0xFF  |  0xFE  |  0xFD  |  0xFC  |  0xFB  |  0xFA  |  0xF9  |  0xF9  | -> End-Of-Transmission Block
```