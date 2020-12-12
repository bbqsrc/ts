# ts

Convenient, small, cross-platform command line tool for generating timestamps on the command line.

## Install

```
cargo install ts-bin
```

## Usage

`ts` prints an ISO 8601 UTC date and time string by default (e.g. `2020-12-12T11:10:40.229006Z`).

It supports the following flags:

- `-l`: prints a local time, like `2020-12-12T12:10:38.407088+01:00`
- `-u`: prints a Unix epoch timestamp, like `1607771679`
- `-H`: as above, but in hexadecimal format, like `5fd4a630`

## License

The `ts-bin` crate is licensed under

* European Union Public License, version 1.2 or later, ([LICENSE](LICENSE) or https://joinup.ec.europa.eu/collection/eupl/eupl-text-11-12)

The EUPL is a **copyleft, GPL-compatible license** managed by the European Union, with legally-equal translated versions **in all languages of the EU**. See [this introduction](https://joinup.ec.europa.eu/collection/eupl/introduction-eupl-licence) for information about the purpose, objectives and translations of the license. See also the [license compatibility matrix](https://joinup.ec.europa.eu/collection/eupl/matrix-eupl-compatible-open-source-licences).