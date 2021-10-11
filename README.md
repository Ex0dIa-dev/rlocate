### rlocate

****

**Build**

```bash
cargo build --release
```

**Use**

```bash
rlocate [FLAGS] <PATTERN>
```

**Example**

```bash
rlocate password
rlocate -i PasSWoRd
rlocate -c password
rlocate -c -i PaSsWoRD
```

****

**Flags**

- **'-c'** ==> *only print number of found entries*
- **'-i'** ==> *ignore case distinctions when matching patterns*
- **'-n'** ==> *print the found entries without colored string*

****

**TODO**

- [x] *count*
- [x] *ignore case*
- [ ] *regexp as pattern*
- [ ] *match only the basename*
- [x] *colored output matches*
