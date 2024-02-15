## `⌽` MicroDB [In Development]
Serialized database for MicroControllers with memory efficiency in mind, requiring only 1KB utilization on the heap regardless of the database size.


#### `⤷` Notes
```
Supports [no_std] with the "embedded" feature that requires an allocator.

Embedded Allocator by [jfrimmel]:
https://github.com/jfrimmel/emballoc

This repository includes a modified version of emballoc for testing purposes.
```


#### `⤷` Implementation
```
To use in a [no_std] environment, you must provide implementations for:
- FileTrait
- OpenFileTrait
- CPathTrait
Found in 'traits.rs'

This is already provided in an std environment, see 'impls.rs'.
```


#### `⤷` Build Features
```
Usable Features:
- std
- embedded [no-std + alloc]

All Features:
- std
- embedded [no-std + alloc]
- no-std
- alloc
```


___
### `➢` Structure
```
Entry UID ─ [32-Bit Fixed-Size Integer]
The unique unstable ID for each database entry encoded in Little Endian byte order,
and is always incrementing sequentially regardless if entries are removed.


EOE: End-Of-Entry ─ [32-Bit Fixed-Size Integer]
An arbitrary series of known bytes to indicate the end of an entry.
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
|  0xC2  |  0xB5  |  0x64  |  0x62  | -> EOE Block
|      > SECOND ENTRY CHUNK <       |
|  0x01  |  0x00  |  0x00  |  0x00  | -> Entry UID
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  XXXX  |  XXXX  |  XXXX  |  XXXX  | -> Serialized Data
|  0xC2  |  0xB5  |  0x64  |  0x62  | -> EOE Block
```

___
### `➢` License
```
This project is licensed under the MIT License.
See the LICENSE file for more information.
```
