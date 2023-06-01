# Kumi v0.1.0, interpreted programming language writen in rust
## Changelog
- Arithmetic operations like +, -, *, /, %, ^, (, )
- Logical operations like ||, &&, !, !=, ==
- Variables with `let` keyword
## Crates
## Examples
```kumi
let a = (1 + 2) * 4
let b = 7 - 3 * (4-(-1))

a==b
```
## Build
1. To build an interpreter run default cargo build command like `cargo build --release`
2. Run interpreter you just builded without cli args like `kumi.exe` or `./kumi` for linux
3. Enter your queries