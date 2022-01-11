#![feature(proc_macro_span)]
#![doc = include_str!("../README.md")]

extern crate proc_macro;

use std::io::Read;

use proc_macro::TokenStream;
use quote::quote;
use relative_path::RelativePath;

mod args;

use args::{Args, Error, PresumedPathToTex, TryGetArgs};

// At the time of writing, proc_macro doesn't offer a way to obtain call site purely
//
// Read about pure and impure functions functions here:
// https://en.wikipedia.org/wiki/Pure_function
fn impurely_get_call_site_dir() -> std::path::PathBuf {
    let call_site: std::path::PathBuf = proc_macro::Span::call_site().source_file().path();
    call_site
        .parent()
        // unwrap won't panic because even if a crate is placed in root,
        // rust files are stored in crate_folder/src/
        .unwrap()
        // It's very likely that there will be unnecessary copying as
        // compilers struggle to perform allocation-related optimizations
        .to_path_buf()
}

impl PresumedPathToTex {
    // The function is called canonicalize_... and not try_canonicalize_... to be consistent with
    // std::fs::canonicalize that also returns std::io::Result<T>.
    fn canonicalize_with_respect_to_call_site_dir(
        self,
        call_site_dir: std::path::PathBuf,
    ) -> std::io::Result<CanonicalizedPresumedPathToTex> {
        // The presumed path to tex has to be made absolute first because only then
        // std::fs::canonicalize() will resolve the path with respect to the macro call site dir
        // and not with respect to the current working directory
        let abs_presumed_path_to_tex: std::path::PathBuf = if self.0.is_relative() {
            let rel_path_from_call_site_to_presumed_tex_file =
                RelativePath::new(self.0.as_os_str().to_str().unwrap());
            rel_path_from_call_site_to_presumed_tex_file.to_logical_path(call_site_dir)
        } else {
            self.0
        };

        abs_presumed_path_to_tex
            .canonicalize()
            .map(CanonicalizedPresumedPathToTex)
    }
}

struct CanonicalizedPresumedPathToTex(std::path::PathBuf);

trait TryReadAsTexFile {
    fn try_read_as_tex_file(self) -> std::io::Result<TexFileContents>;
}

impl TryReadAsTexFile for std::io::Result<CanonicalizedPresumedPathToTex> {
    fn try_read_as_tex_file(self) -> std::io::Result<TexFileContents> {
        let canonicalized_presumed_path_to_tex: CanonicalizedPresumedPathToTex = self?;
        let path = canonicalized_presumed_path_to_tex.0.as_path();
        let mut file = std::fs::File::open(path)?;
        let mut buffer: String = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(TexFileContents(buffer))
    }
}

struct TexFileContents(String);

impl TexFileContents {
    fn into_markdown_tex(self) -> MarkdownTex {
        let backslash_count = self.0.chars().filter(|c| c == &'\\').count();
        let mut escaped_string_buffer = String::with_capacity(backslash_count + "$$$$".len());
        escaped_string_buffer += "$$";
        for c in self.0.chars() {
            core::iter::repeat(())
                .take(if c == '\\' { 2 } else { 1 })
                .for_each(|_| escaped_string_buffer.push(c));
        }
        escaped_string_buffer += "$$";
        MarkdownTex(escaped_string_buffer)
    }
}

struct MarkdownTex(String);

trait IntoTokenStream {
    fn into_token_stream(self) -> TokenStream;
}

impl IntoTokenStream for std::io::Result<MarkdownTex> {
    fn into_token_stream(self) -> TokenStream {
        match self {
            Ok(markdown_tex) => {
                let s = markdown_tex.0;
                quote! { #s }
            }
            Err(e) => {
                let error_message = e.to_string();
                quote! { #error_message }
            }
        }
        .into()
    }
}

/// Read the crate documentation for details
#[proc_macro]
pub fn include_display_mode_tex(ts: TokenStream) -> TokenStream {
    let Args{ presumed_path_to_tex } = match ts.try_get_args() {
        Ok(args) => args,
        Err(e) => {
            return match e {
                Error::TheMacroAcceptsOnlyOneArgument => quote! {
                    compile_error!(concat!(stringify!(include_display_mode_tex), " accepts only one argument")) 
                },
                Error::TheMacroExpectedAnArgument => quote! {
                    compile_error!(concat!(stringify!(include_display_mode_tex), " expected an argument, specifically a string literal")) 
                },
                Error::TheOnlyArgumentMustBeAStringLiteral => quote! {
                    compile_error!(concat!(stringify!(include_display_mode_tex), " expected a string literal as an argument")) 
                }
            }.into()
        }
    };

    presumed_path_to_tex
        .canonicalize_with_respect_to_call_site_dir(impurely_get_call_site_dir())
        // No validation occurs because it would slow down compilation times
        .try_read_as_tex_file()
        .map(TexFileContents::into_markdown_tex)
        .into_token_stream()
}
