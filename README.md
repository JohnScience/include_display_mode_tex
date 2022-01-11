# What is offered by this crate

The crate provides `include_display_mode_tex!` macro that allows to embed tex formulae
in documentation generated by `rustdoc` via `cargo doc`.

# Requirements

* Nightly compiler because the code requires [`#![feature(proc_macro_span)]`](https://github.com/rust-lang/rust/issues/54725)

# What `include_display_mode_tex!` can do now VS what can be done theoretically
In its current implementation, `include_display_mode_tex!` macro merely turns
the contents of `.tex` files into Markdown with raw LaTeX formulae. For the formulae to be displayed 
as such and not LaTeX syntax, Markdown with raw LaTeX must be rendered with some library, such as
[`KaTeX`](https://katex.org/docs/autorender.html) or
[`MathJax`](http://docs.mathjax.org/en/latest/web/configuration.html). Such approach burdens the crate
with extra complexity of `.cargo` config and the requirement to build the documentation via 
`cargo doc --no-deps` instead of `cargo doc` **but it works**.

There is also [`katex` crate](https://docs.rs/katex/latest/katex/) that theoretically can allow
to render HTML when the documentation is generated. A PR with such functionality will be very
welcome (though feature-gated for backward compatibility).

# Setting up the crate (for using `include_display_mode_tex!` with [`KaTeX` renderer](https://katex.org/docs/autorender.html))

1. Create `.cargo` directory in the crate root (the directory containing `Cargo.toml`)
2. In `.cargo`, add [`config.toml`](https://doc.rust-lang.org/cargo/reference/config.html)
with the following contents:
```toml
[build]
rustdocflags = [ "--html-in-header", "./src/html/docs-header.html" ]
```
3. Add these two line to your `Cargo.toml`
```toml
[package.metadata.docs.rs]
rustdoc-args = [ "--html-in-header", "./src/html/docs-header.html" ]
```
4. Create `./src/html` directory (where `./src/html` is a relative path from the crate root)
5. In `./src/html` add `docs-header.html` with the following contents:
```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.15.1/dist/katex.min.css" integrity="sha384-R4558gYOUz8mP9YWpZJjofhk+zx0AS11p36HnD2ZKj/6JR5z27gSSULCNHIRReVs" crossorigin="anonymous">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.15.1/dist/katex.min.js" integrity="sha384-z1fJDqw8ZApjGO3/unPWUPsIymfsJmyrDVWC8Tv/a1HeOtGmkwNd/7xUS0Xcnvsx" crossorigin="anonymous"></script>
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.15.1/dist/contrib/auto-render.min.js" integrity="sha384-+XBljXPPiv+OzfbB3cVmLHf4hdUFHlWNZN5spNQ7rmHTXpd7WvJum6fIACpNNfIR" crossorigin="anonymous"></script>
<script>
    document.addEventListener("DOMContentLoaded", function() {
        renderMathInElement(document.body, {
            delimiters: [
                {left: "$$", right: "$$", display: true},
                {left: "\\(", right: "\\)", display: false},
                {left: "$", right: "$", display: false},
                {left: "\\[", right: "\\]", display: true}
            ]
        });
    });
</script>
```
# Example
```no_run
use include_display_mode_tex::include_display_mode_tex;

#[doc = include_display_mode_tex!("./tex/example.tex")]
# let s = 0;
```

Notice that the path is relative not to the crate root but to the call site and that
the documentation must be built with 
```text
cargo doc --no-deps
```

# Sources of inspiration
Other include\* macros:
* [`core::include_str`](https://doc.rust-lang.org/core/macro.include_str.html)
* [`core::include_bytes`](https://doc.rust-lang.org/core/macro.include_bytes.html)
* [`core::include`](https://doc.rust-lang.org/core/macro.include.html)
* [`include_dir::include_dir`](https://crates.io/crates/include_dir)

Special thanks to [`victe`](https://github.com/victe) for providing 
[`rust-latex-doc-minimal-example`](https://github.com/victe/rust-latex-doc-minimal-example)

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>