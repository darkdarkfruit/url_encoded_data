#!/usr/bin/env bash

## https://docs.rs/cargo-readme/3.2.0/cargo_readme/
#> cargo readme
#
#If you have additional information that does not fit in doc comments, you can use a template. Just create a file called README.tpl in the same directory as Cargo.toml with the following content:
#
#{{badges}}
#
## {{crate}}
#
#{{readme}}
#
#Current version: {{version}}
#
#Some additional info here
#
#License: {{license}}
cargo readme  > README.md