# amidir

`amidir` is a rewrite of `amidi` from the 
[alsa-utils project](https://github.com/alsa-project/alsa-utils) 
in Rust, just to see how it is done.

This project attempts to implement the `amidi` utility
from the ALSA utilities in Rust. The intention is not
to take anything away from the original `amidi` or
ALSA, but to explore the developer experience in Rust
as compared to the C language.

`amidir` relies on the [`alsa`](https://crates.io/crates/alsa) crate,
which provides a safe Rust wrapper around the ALSA API.

The parsing of the command line arguments relies on
the [`clap`](https://crates.io/crates/clap) crate.

## License

The original `amidi` is licensed under GPL-2.0.
This program does not include the original utility,
and is written in a different programming language,
so it can not be considered as a derivative work.

Therefore this program is licensed under the MIT license,
which is more common, and arguably more suitable
for use in the Rust ecosystem.

However, the program attempts to emulate the workings and usage
of the original utility. This follows the precedent
set by author Ken Youens-Clark in the 
[Command-Line Rust](https://github.com/kyclark/command-line-rust)
project, also licensed under the MIT license.
