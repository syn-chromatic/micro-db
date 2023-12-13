## `⌽` MicroDB [In Development]
Serialized database for Microcontrollers with memory efficiency in mind.


#### `⤷` Notes
```
Supports [no_std] with the "embedded" feature but requires an allocator.

Embedded Allocator by [jfrimmel]:
https://github.com/jfrimmel/emballoc

This repository includes a modified version of emballoc for testing purposes.
```


#### `⤷` Current Performance (not tested on an MCU)
```
Iterates over an entire 7MB database file containing 100K entries
while only utilizing 1KB on the heap and takes 58ms (0.58μs per entry)
using a cache size of 512 bytes.
```


___
### `➢` Structure (Not Finalized)
```
---
Entry UID ─ [32-Bit Fixed-Size Integer]
The unique ID for each database entry encoded in Little Endian byte order, and is always
incrementing sequentially regardless if entries are removed.

----
EOE: End-Of-Entry ─ [32-Bit Fixed-Size Integer]
An arbitrary series of known bytes to indicate the end of an entry (needs more research).
```


#### `⤷` 4-byte aligned database structure
```
|   00   |   01   |   02   |   03   |
|--------|--------|--------|--------|
|       > FIRST ENTRY CHUNK <       |
|  0x00  |  0x00  |  0x00  |  0x00  | -> Entry UID
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  0xFF  |  0xFE  |  0xFD  |  0xFC  | -> EOE Block
|      > SECOND ENTRY CHUNK <       |
|  0x01  |  0x00  |  0x00  |  0x00  | -> Entry UID
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  0xFF  |  0xFE  |  0xFD  |  0xFC  | -> EOE Block
```


___
### `➢` Known Issues

- ~~Serialized data with variable entry chunk sizes muddles the database on removal of entry~~ ─ Issue Fixed


___
### `➢` License
```
This project is licensed under the MIT License.
See the LICENSE file for more information.
```