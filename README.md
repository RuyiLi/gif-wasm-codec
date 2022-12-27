# gif-wasm-codec

A WebAssembly library to encode and decode gifs in the browser.

This project is essentially a wrapper over the [gif crate](https://crates.io/crates/gif) written in Rust,

## Notes

This project is incomplete - many options exposed by the underlying crate are not configurable. However, the default options should be sufficient for most use cases (like in my [image splitter](https://github.com/RuyiLi/image-splitter)).

Decoding works well and is faster than most JS-only libraries that I've tested. Encoding, on the other hand, has not been fully optimized/may have memory issues and until WASM is more mature I don't see it being faster than a pure JS + web workers approach.

## Usage

Install the library from npm.

```
yarn add gif-wasm-codec
```

Import the library.

```
import * as gif from 'gif-wasm-codec'
```

## API

Using this library at the moment requires a bit of manual work; my focus has been on correctness and optimization, so I haven't built out any abstractions yet.

`decode(arr: Uint8Array) -> DecodedGif` takes a [Uint8Array](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) representing an image and returns a `DecodedGif` object.

`DecodedGif` is the interface representing the outcome of a call to `decode`. Contains three items:

- `num_frames` is the number of frames in the gif.
- `metadata` is a flattened array of five-tuples in the form of `(delay, top, left, width, height)`. The slice `metadata[5i..5i+5]` represents these values for the `i`th frame.
- `frames` is a flattened array of the image data for all frames. Note that with some compression techniques, not every frame will be the same size. We use the metadata to compute the size: `width * height * 4` (RGBA).

Example: View all frames of an uploaded gif.

```ts
const fileInput = ... // Some input with type file
fileInput.addEventListener("input", async function () {
  const file = decodeInput.files![0];
  const buf = await file.arrayBuffer();
  const results = gif.decode(new Uint8Array(buf))

  let frameIdx = 0
  for (let i = 0; i < num_frames; i++) {
    const [
      delay,
      top,
      left,
      width,
      height
    ] = metadata.subarray(
      i * 5,
      (i + 1) * 5
    );

    const size = width * height * 4;
    const frame = frames.subarray(i, i + size);
    frame_idx += size;

    const imgData = new ImageData(frame, width, height);
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d")!;
    canvas.width = left + width;
    canvas.height = top + height;
    ctx.putImageData(imgData, left, top);
    document.body.append(canvas);
  }

  results.free()
})
```

`Encoder` is not recommended for use. Use something like gif.js instead.
