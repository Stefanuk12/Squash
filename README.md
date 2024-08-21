A library for serializing and deserializing binary data with a focus on size and Roblox compatibility. It is designed with the programmer in mind, offering full flexibility and control over the data being serialized and deserialized, with an intuitive API.

## Implementation details

You can view how [Squash](https://data-oriented-house.github.io/Squash/) was implemented [here](https://data-oriented-house.github.io/Squash/docs/how). It was initially a Luau library with Roblox in mind, but now ported to Rust!

There is built-in `serde` support for Squash, but there is also a [custom API](./squash/src/serdes/cursor/mod.rs) for more control over the serialization and deserialization process.

## Examples

You can view examples on how to use Squash [here](./squash/examples).