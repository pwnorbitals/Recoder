#  Recoder :loop:
Rust-based generic encoder/decoder/hasher 

This is my first real project using Rust. I hope to be able to support many encodings and hashes and make the tool as easy-to-use as possible. Please open an issue if the encoder you want or need isn't implemented.

Feel free to give feedback on the code and send pull requests :)


## Usage

Simple encoding :
`./recoder --encode base64 RustIsAwesome` 

Chaining :
`./recoder -e base64 RustIsAwesome | ./recoder -d base64`

## Options

`-e`, `--encode NAME` : encode / hash the data using the given encoder. Mutually exclusive with `-d`. At least one of `-e`, `-d` must be supplied.

`-d`, `--decode NAME` : decode the data (when possible) using the given decoder.

`-o`, `--options [OPTS]` : encoder-specific options. Options are parsed as a query string : `key=value&key2=value2&...`

## Encoders

| Names | Type | Options |
|-------|---------|-------------|
|"md5", "md-5", "MD5" | hasher | None |
|"sha1", "sha-1", "SHA1" | Hasher | None |
|"base64", "b64", "BASE64" "B64" | Encoder | None |
|"urlencode", "url", "urlencoding" "URL" | Encoder | None |
|"sha256", "SHA256", "sha2" | Hasher | None |
|"hex", "HEX" | Encoder | `ignore-whitespace=[true/false]`, `separator=(string)`  |

## Work-in-progress

- [ ] Common options for encoders

- [ ] More encoders and hashers 

    - [ ] decimal

    - [ ] octal

    - [ ] offset

    - [ ] caesar

    - [ ] sha-256 variants

    - [ ] sha-512 and variants

    - [ ] bcrypt

    - [ ] scrypt

- [ ] Write tests
