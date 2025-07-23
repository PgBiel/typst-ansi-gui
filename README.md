# typst-ansi-gui
Small GUI to highlight some Typst code using ANSI escape codes with [@frolozotl](https://github.com/frozolotl)'s amazing https://github.com/frozolotl/typst-ansi-hl

(**UPDATE:** Also check out `typst-ansi-web` by the author: https://pgbiel.github.io/typst-ansi-web/)

Made with [iced](https://iced.rs/).

You can **try it in your browser:** https://pgbiel.github.io/typst-ansi-gui/

![app screenshot](https://github.com/PgBiel/typst-ansi-gui/assets/9021226/7a60c269-b78b-4ee6-a0a9-93966fa61d36)

## Usage

Run the application, paste your Typst code at the top, and press `Highlight`. Then, press `Copy` to copy the generated ANSI.

You can then paste the copied highlighted code on Discord as such:

````
```ansi
PASTE IT HERE
```
````

You should now see your Typst code with proper syntax highlighting.

## Installing

You can run `cargo install --git https://github.com/PgBiel/typst-ansi-gui --locked typst-ansi-gui`, which will build the application from source and install.

## Building

Just run `cargo build` and the built application will be at `target/debug/typst-ansi-gui`. You can then run it.

## Formatting and Checking

To format the code, run `cargo fmt`. To check, run `cargo clippy`.

## License

Licensed under MIT or Apache-2.0, at your choice.
