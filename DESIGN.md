# Provided items

The crate is meant to provide only one item, its namesake, `include_display_mode_tex!`() macro.
This macro must embed [display mode](http://www.malinc.se/math/latex/inlinedisplayen.php) tex.

# How it works

## High-level explanation

First and foremost, the only macro of this crate, `include_display_mode_tex!`() , is
implemented as a [function-like procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros), i.e. it accepts and outputs [`TokenStream`] and is driven by
the machinery provided by the [`proc_macro` crate](https://doc.rust-lang.org/reference/procedural-macros.html#the-proc_macro-crate)

## The procedure

1. The macro processes the input [`TokenStream`]:
   1. First, the macro turns [`TokenStream`] into iterator over [`TokenTree`]s
```rust
// include_display_mode_tex/src/args.rs

use proc_macro::{token_stream::IntoIter as TokenTreeIter /*...*/}

// ...
impl ArgsTokenStrean {
    // ...

    fn extract_arg_token_trees(self) -> core::result::Result<ArgTokenTrees, Error> {
        let mut arg_tt_iter: TokenTreeIter = self.0.into_iter();
        // ...
    }
}
```
   1. Then, it assigns the first [`TokenTree`]\(s\) from the iterator
1. 

# Plans for the development

## Specifying the path from canonical directories

### Problem

At the time of writing the macro accepts only a string literal that contains either an
absolutele path to a tex file to be embedded or a **relative path from the call site directory**.
While this is roughly the behavior of [`include_str!`()](https://doc.rust-lang.org/std/macro.include_str.html),
it might be not very convenient for some developers.

If a developer wants to keep `.tex` files separate from `.rs` files but the depth of modules
is very high, they will quickly find themselves writing something like this:

```rust
// The story goes the developer will eventually start typing the path from the crate root
#[doc = include_display_mode_tex!("../../../../../../../")]
```

### Suggested solution

In order to make the developer experience more pleasant, the alternative syntax should be

```rust
#[doc = include_display_mod_tex!("CARGO_MANIFEST_DIR/path/from/crate_root/tex_file.tex")]
```

for the path from the root of the crate where the macro was called and

```rust
#[doc = include_display_mod_tex!("TEX_DIR/path/to/tex_file.tex")]
```

for the path from the `CARGO_MANIFEST_DIR/src/tex/` where, just like earlier, `CARGO_MANIFEST_DIR`
is the root of the crate where the macro was called.

#### Choice of names

* The name `CARGO_MANIFEST_DIR` can be already familiar to Rust developers as
[the name of the environment variable that Cargo sets for crates](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates)

* The name `TEX_DIR` is a novelty that was constructed by analogy and is expected be easy to remember

### Drawbacks

The offered solution can cause some negligible compatibility issues if the developer for some
unknown reason used either `CARGO_MANIFEST_DIR` or `TEX_DIR` as actual directory names.

[`TokenStream`]: https://doc.rust-lang.org/proc_macro/struct.TokenStream.html
[`TokenTree`]: https://doc.rust-lang.org/proc_macro/enum.TokenTree.html#