[![crates.io](https://img.shields.io/crates/v/include_display_mode_tex.svg)][`include_display_mode_tex`]
[![crates.io](https://img.shields.io/crates/d/include_display_mode_tex.svg)][`include_display_mode_tex`]
[![crates.io](https://img.shields.io/github/workflow/status/JohnScience/include_display_mode_tex/Rust)][`include_display_mode_tex`]

# What is offered by this crate

The crate provides [`include_display_mode_tex!`](https://docs.rs/include_display_mode_tex/latest/include_display_mode_tex/macro.include_display_mode_tex.html) macro that allows to embed tex formulae
in the documentation generated by `rustdoc` via `cargo doc --no-deps`.

# Requirements

* Nightly compiler

[`#![feature(proc_macro_span)]`](https://github.com/rust-lang/rust/issues/54725) is absolutely necessary
to imitate the behavior of [include_str](https://doc.rust-lang.org/core/macro.include_str.html).

Running `rustup default nightly` in the terminal will install the nightly version of the tools (cargo, rustc, and so on). Also, it will switch the corresponding commands to use the nightly version. If you want to go back to the stable version, issue the `rustup default stable` command. [Credit to O'Reilly](https://www.oreilly.com/library/view/rust-programming-by/9781788390637/e07dc768-de29-482e-804b-0274b4bef418.xhtml).

# What is LaTeX?

LaTeX is a language for typesetting documents, especially scientific papers, and a document preparation system.

## Example of .tex code

```tex
% ...
\subsection*{H}
	\glossaryentry{hadamard_product}{Hadamard product}
	\begin{adjustwidth}{1em}{}
		\textbf{Field of study}: \textit{Mathematics. Linear Algebra. Matrix theory.} \\
		\textbf{Distinct meanings in other fields of study}: \textit{unspecified.} \\
		\textbf{Definitions}:
		\begin{adjustwidth}{1em}{} \leavevmode
			\begin{framed}
				For two \hyperlink{matrix}{\textit{matrices}} $A$ and $B$ of the same \hyperlink{dimension_of_matrix}{\textit{dimension}} $m \times n$, the \beingdefined{Hadamard product} $A \circ B$ (or $A \odot B$) is a \hyperlink{matrix}{\textit{matrix}} of the same \hyperlink{dimension_of_matrix}{\textit{dimension}} as the operands, with elements given by
				\begin{equation*}
					(A \circ B)_{ij} = (A \odot B)_{ij} = (A)_{ij}(B)_{ij}.
				\end{equation*}
				
				Source: \cite{wiki_hadamard_product_matrices}.
			\end{framed}
			\begin{framed}
				Let $A$ and $B$ be $m \times n$ \hyperlink{matrix}{\textit{matrices}} with entries in $C$. The \beingdefined{Hadamard product} is defined by $[A \circ B]_{ij}=[A]_{ij}[B]_{ij}$ for all $1 \leq i \leq m$, $1 \leq j \leq n$. \\ \vspace{1em}
				
				Source: \cite{emillion}.
			\end{framed}
		\end{adjustwidth}
	\end{adjustwidth} \vspace{1em}
% ...
```

### Output

![tex output](https://i.imgur.com/xptzo3h.jpg)

# Example on docs.rs

[`zero_based_index` crate](https://docs.rs/zero_based_index/latest/zero_based_index/struct.ZBI.html)

* [GitHub](https://github.com/JohnScience/zero_based_index)
* [crates.io](https://crates.io/crates/zero_based_index)

# Now and future

In its current implementation [`include_display_mode_tex!`](https://docs.rs/include_display_mode_tex/latest/include_display_mode_tex/macro.include_display_mode_tex.html) macro merely turns
the contents of `.tex` files into [Markdown](https://en.wikipedia.org/wiki/Markdown) with raw LaTeX formulae. For the formulae to be displayed as such and not LaTeX syntax, [Markdown](https://en.wikipedia.org/wiki/Markdown) with raw LaTeX must be rendered with some library, such as
[`KaTeX`](https://katex.org/docs/autorender.html) or
[`MathJax`](http://docs.mathjax.org/en/latest/web/configuration.html). Such approach burdens the crate
with extra complexity of `.cargo` config and the requirement to build the documentation via 
`cargo doc --no-deps` instead of `cargo doc` **but it works**.

There is also [`katex` crate](https://docs.rs/katex/latest/katex/) that theoretically can allow
to render HTML when the documentation is generated. A PR with such functionality will be very
welcome (though feature-gated for backward compatibility).

# Setting up the crate

The following steps will allow to render `.tex` included via `include_display_mode_tex!`
with [`KaTeX` renderer](https://katex.org/docs/autorender.html):

1. Create `.cargo` directory in the crate root (the directory containing `Cargo.toml`)
> This is needed for config.toml
2. In `.cargo` directory, add [`config.toml`](https://doc.rust-lang.org/cargo/reference/config.html)
with the following contents:
```toml
[build]
rustdocflags = [ "--html-in-header", "./src/html/docs-header.html" ]
```
> This rustdoc flag will ensure that the documentation generated by rustdoc, which is built with HTML and CSS, will have our HTML header containing necessary CSS and JavaScript for rendering LaTeX.
3. Add these two lines to your `Cargo.toml`
```toml
[package.metadata.docs.rs]
rustdoc-args = [ "--html-in-header", "./src/html/docs-header.html" ]
```
> Since docs.rs is a separate website where crate documentation is published, it checks only the metadata that concerns docs.rs. So we have to add it in the metadata as well.
4. Add `include_display_mode_tex` as an optional `[dependency]`, not `[dev-dependency]` in `Cargo.toml`:
```toml
[dependency]
include_display_mode_tex = { version = "<version>", optional = true }
```
5. Add `doc` to the list of `[features]` in `Cargo.toml` and make it enable compilation of `include_display_mode_tex` crate:
```toml
# such feature is needed in order to avoid compilation of nightly `include_display_mode_tex` which is
# used only for documentation generation
doc = ["include_display_mode_tex"]
```
6. Create `./src/html` directory (where `./src/html` is a relative path from the crate root)
7. In `./src/html` add `docs-header.html` with the following contents:
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
> KaTeX can occasionally get updated. Also, you can other libraries instead.

# Code example
```no_run
#[cfg(all(doc, feature = "doc"))]
use include_display_mode_tex::include_display_mode_tex;

#[cfg_attr(all(doc, feature = "doc"), doc = include_display_mode_tex!("./tex/example.tex"))]
let s = 0;
```

Notice that the path is relative not to the crate root but to the call site (just like
for [`core::include_str`](https://doc.rust-lang.org/core/macro.include_str.html)) and that
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

[`include_display_mode_tex`]: https://crates.io/crates/include_display_mode_tex

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
