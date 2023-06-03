# QOI to PNG Converter

This is a decoder for the [QOI image format](https://qoiformat.org), written in Rust. It uses parser combinators to parse any .qoi image as byte chunks, and converts chunks to pixels, which are stored in an [ImageBuffer](https://docs.rs/image/0.24.6/image/struct.ImageBuffer.html). The [image](https://crates.io/crates/image) crate is used to convert the ImageBuffer to a png image.

## Using the decoder

You will need Rust installed on your system to use the decoder. You can do so by using [rustup](https://rustup.rs). Once this is done, clone the repo, navigate to the root directory and install the binary using

```bash

cargo install --path .
```

After this, you can use the decoder

```bash
# converts dice.qoi to dice.png
pngify dice.qoi
```

You can optionally specify a custom filename for the png file

```bash
# converts dice.qoi to testimage.png and stores it at the specified path
pngify dice.qoi /home/xyz/testimage.png
```

