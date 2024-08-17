murmur-inverse can find 15 character string collisions for [MurmurHash64A](https://en.wikipedia.org/wiki/MurmurHash#MurmurHash2) hashes.

When modding Stingray games the inverse hash is used to reference resources
where the original path is unknown.
However, the Vermintide 2 SDK doesn't support unicode/hex escaping in strings
which is necessary to use inverse hashes.
In those cases a string collision can be find with murmur-inverse.

murmur-inverse is fast enough to find collisions for a million hashes in under a second.

### Examples

Pass hash as argument:
```
murmur-inverse 15c2e52cd7358dcd
```

Or a file as argument:
```
murmur-inverse hashes.txt
```

with `hashes.txt` containing:
```
15c2e52cd7358dcd
0d71920874942abe
b85fde08cde3ac1b
```

Pipe output to file if needed:
```
murmur-inverse hashes.txt > keys.txt
```
