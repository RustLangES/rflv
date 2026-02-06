# RFLV

Rflv is an encoder-decoder for FLV written purely in Rust.

Rflv can create/read FLV files, and can also directly generate FLV tags in cases where the user works directly with FLV tags (e.g., RTMP).

# What RFLV does not do?

RFLV is simply an implementation of FLV for Rust; it is not an implementation of a decoder/encoder for any specific codec. The common workflow would be to have a separate codec and pass what the codec gives to RFLV.

# Little Example:
```
decode:
let stream = ...; // must implement WriteBytesExt/ReadBytesExt of byteorder
let tag = FlvTag::decode(&mut stream)?;


```
