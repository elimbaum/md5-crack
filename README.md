# md5-crack
simple md5 cracker in rust.

Usage:

    cargo run --releaese [hash]

## Examples

| plaintext | md5     						   |
| :-------- | :------------------------------- |
| abc 		| 900150983cd24fb0d6963f7d28e17f72 |
| dog 		| 06d80eb0c50b49a509b49f2424e8c805 |
| yelp 		| 771c0159ac754c62cdc1c5981d1412f9 |
| free 		| aa2d6e4f578eb0cfaba23beef76c2194 |
| fred 		| 570a90bfbf8c7eab5dc5d4e26832d5b1 |
| apple 	| 1f3870be274f6c49b3e31a0c6728957f |
| hOrSe 	| 55ec8e764ce197dac6c7ae83dbf1c5bf |
| snake 	| de1b2a7baf7850243db71c4abd4e5a39 |
| pandas 	| 3a43b4f88325d94022c0efa9c2fa2f5a |
| password  | 5f4dcc3b5aa765d61d8327deb882cf99 |

## TODO
  - give each thread its own comparison hash
  - try with actual threads, real speedup?
  - figure out order of iterator processing
  - thread local counters; add at end
  - don't use strings - the back and forth is slow
