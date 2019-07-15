# Recoder
Rust-based generic encoder/decoder/hasher 

This is my first real project using Rust. I hope to be able to support many encodings and hashes and make the tool as easy-to-use as possible.

Feel free to give feedback on the code and send pull requests :)


## Usage

Simple encoding :
`./recoder --encode base64 RustIsAwesome` 

Chaining :
`./recoder -e base64 RustIsAwesome | ./recoder -d base64` 