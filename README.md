# findschemajson

This is a tool to determine the schema of specific keys when you have
millions of JSON documents in a file.

Given entries.log has input:

```
{"randkey":123}
{"randkey":"123"}
{"randkey":"hi there"}
```

```bash
cat entries.log | ./target/release/findschemajson randkey
```

will output:

```bash
randkey: {String, Number, CastableStringToNumber}
```

Meaning, that randkey has incompatible value types if you were to
analyze this data. The string ```hi there``` cannot be cast to a
number. The tool has very good performance but it can degrade as the
number of keys passed to the tool increases.

## Compilation

Make sure you have the rust compiler and cargo installed. Otherwise,
follow the instructions in https://www.rust-lang.org/tools/install

```bash
cargo build --release
```

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
