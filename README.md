Brute forces a 15 character string to match a [MurmurHash64A](https://en.wikipedia.org/wiki/MurmurHash#MurmurHash2) hash.

### Examples

Pass hash as argument:
`murmur-inverse 15c2e52cd7358dcd`

Or a text file:
`murmur-inverse hashes.txt`

with `hashes.txt` containing:
```
15c2e52cd7358dcd
0d71920874942abe
b85fde08cde3ac1b
```

Pipe output to file if needed:
`murmur-inverse hashes.txt > keys.txt`
