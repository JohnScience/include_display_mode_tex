# Provided items

The crate is meant to provide only one item, its namesake, `include_display_mode_tex!`() macro.

# Plans for the development

## Specifying the path from canonical directories

### Problem

Even though at the time of writing the macro accepts only one argument, a string literal 
that contains either an absolute path to or a relative path with respect to the call site directory,
it might be not very convenient for some developers.

If a developer wants to keep `.tex` files separate from `.rs` files but the depth of modules
is very high, they will quickly find themselves writing something like this

```rust
// The story goes the developer will eventually start typing the path from the crate root
#[doc = include_display_mode_tex!("../../../../../../../")]
```

### Suggested solution

In order to make the developer experience more pleasant, the alternative syntax should be

```rust
#[doc = include_display_mod_tex!("CARGO_MANIFEST_DIR/path/from/crate_root/tex_file.tex")]
```

for the path from the root of the crate the the macro was called and

```rust
#[doc = include_display_mod_tex!("TEX_DIR/path/to/tex_file.tex")]
```

for the path from the `CARGO_MANIFEST_DIR/src/tex/` where, just like earlier, `CARGO_MANIFEST_DIR`
is the root of the crate where the macro was called.

### Drawbacks

The offered solution can cause some negligible compatibility problems if the developer for some
unknown reason used either `CARGO_MANIFEST_DIR` or `TEX_DIR` as actual directory names.