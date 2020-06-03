# Kakusu
Kakusu is a small toy project that allows you to hide a UTF-8 text file within an image.

## Requirements
Rust 2018+ is required for building the project. 

## Setup
Clone this project, `cd` into the directory and type

`cargo build --release`

The program is found within the `target/release` folder that was created from the previous `cargo build` command.

## Examples
**Encoding a message in an image**:

`/path/to/kakusu_target_release/kakusu -f="message.txt" -i="/path/to/image"`

or

``/path/to/kakusu_target_release/kakusu -m="This is a secret message!" -i="/path/to/image"``

The previous commands encodes the image and saves into a new file called `secret.png` in the same directory you ran `kakusu`.

**Decoding an image**:

`/path/to/kakusu_target_release/kakusu -i="/path/to/secret.png" -d`

This will print out the message to your terminal. If no secret message is found, it will print out a UTF-8 parsing error.
