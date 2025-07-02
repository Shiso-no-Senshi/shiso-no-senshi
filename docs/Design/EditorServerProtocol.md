# Editor Server Protocol

The Editor Server Protocol (ESP) is an evolution of the [Language Server Protocol v3.17](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/) to support both binary and text content.

## Message Format

All messages are formatted as [Javascript Object Notation  (JSON)](https://www.json.org/) text using UTF-8 encoding and Unix style (`\n`) line endings. Since "pretty printing" of JSON is trivial, all unnecessary characters and whitespace should be omitted from the transmitted messages. All JSON data types are supported.

A string is a sequence of zero or more UTF-8 encoded Unicode characters, wrapped in double quotes, using backslash escapes. A character is represented as a single character string.

The recognized escapes are:

| Sequence | Value |
| :---: | :---: |
| `\"` | Quotation Mark |
| `\\` | Reverse Solidus (Backslash) |
| `\/` | Solidus (Slash) |
| `\b` | Backspace |
| `\f` | Form Feed |
| `\n` | Line Feed |
| `\r` | Carriage Return |
| `\t` | Horizontal Tab |
| `\uXXXX` | The Unicode character at code point XXXX |

Numbers are written using standard floating point notation, including an leading dash (a.k.a. negative sign), an integral part, a period (a.k.a. decimal point, `.`), a fractional part, the letter `e` (case ignored), and an exponential part. Only the integral part is required.
