# any-file-binder
```
Simple CLI tool for file binding on windows
```

# how to use?
```
Build and just run binder.exe
but u should include `stub.exe` file in your binder.exe folder
```
## how to use stub?
```
just build and push in folder with binder
```

## Stub structure
stub is encrypted with aes-256-cbc
and seperated binary with `__PARSE_STUB__` symbol

and then stub contains binded file data after seperator like this
```
| __PARSE_STUB__ | encrypt type (u8) | file length (u64) | binded file data |
| --------------- | ------------------ | ------------------ | ---------------- |
```

There is 2 type of encrypt type
- 0: AES-256-CBC
- 1: AES-128-CBC

Binded file data binary structure is like this
```
| size (u64) | file name size (u64) | file name | file data (encrypted) |
| ---------- | -------------------- | --------- | ---------------------- |
```

so first 16 bytes is size of encrypted binded file data
and then 16 bytes is file name size
and then file name
and then file data (encrypted)
