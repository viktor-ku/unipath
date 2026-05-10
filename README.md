# Unipath

> An attempt to replace Rust's std Path

## Differences

- Operates on top of bytes `&[u8]` or str slices `&str`
    instead of `OsStr`

## Notes to self

- 32 bytes for filename (22 mean for me)
- 128 for path length (82 mean for me)
