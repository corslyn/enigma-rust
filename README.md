
# enigma-rust

An implementation of the Enigma Machine in Rust

## Run Locally

Clone the project

```bash
  git clone https://github.com/corslyn/enigma-rust
```

Go to the project directory

```bash
  cd enigma-rust
```

Compile and start the program

```bash
$ cargo run -- -h

An implementation of the Enigma Machine in Rust

Usage: enigma-rust [OPTIONS] --rotors <ROTORS> --reflector <REFLECTOR> --text <TEXT>

Options:
  -r, --rotors <ROTORS>        Three rotor settings, e.g., "I II III"
      --reflector <REFLECTOR>  Reflector to use, e.g., "B"
  -p, --plugboard <PLUGBOARD>  Plugboard settings, e.g., "AB CD EF" (optional)
  -k, --key <KEY>              Initial rotor positions, e.g., "A B C" (optional)
  -t, --text <TEXT>            The text to encode or decode
  -h, --help                   Print help
  -V, --version                Print version
```


## Roadmap

- [x] CLI
- [ ] Web interface using actix-web

