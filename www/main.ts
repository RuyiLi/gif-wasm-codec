import * as gif from '../pkg/gif_wasm_codec'
import * as gifuct from 'gifuct-js'

const $ = document.querySelector.bind(document)

function decode1(buf) {
  const { metadata, frames, num_frames } = gif.decode(new Uint8Array(buf))

  let frame_idx = 0
  for (let i = 0; i < num_frames; i++) {
    const [delay, top, left, width, height] = metadata.subarray(i * 5, (i + 1) * 5)
    const size = width * height * 4
    const frame = frames.subarray(frame_idx, frame_idx + size)
    frame_idx += size

    const imgData = new ImageData(frame, width, height)
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d');
    canvas.width = width;
    canvas.height = height;
    ctx.putImageData(imgData, 0, 0);
    document.body.append(canvas)
  }
}

function decode2(buf) {
  const g = gifuct.parseGIF(buf)
  const frames = gifuct.decompressFrames(g, true)

  for (const frame of frames) {
    const canvas = document.createElement('canvas')
    const ctx = canvas.getContext('2d')
    const imgData = ctx.createImageData(frame.dims.width, frame.dims.height)
    imgData.data.set(frame.patch)
    canvas.width = imgData.width;
    canvas.height = imgData.height;
    ctx.putImageData(imgData, 0, 0);
    document.body.append(canvas)
  }
}

const decodeInput = $('#decode-input') as HTMLInputElement
decodeInput.addEventListener('input', async function (evt) {
  const file = decodeInput.files[0]
  const buf = await file.arrayBuffer()

  console.time('decode1')
  decode1(buf)
  console.timeEnd('decode1')

  const hr = document.createElement('hr')
  document.body.append(hr)

  console.time('decode2')
  decode2(buf)
  console.timeEnd('decode2')
})