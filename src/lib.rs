extern crate wasm_bindgen;
extern crate gif;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
  #[wasm_bindgen(js_namespace=console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub struct DecodeResults {
  pub num_frames: usize,
  metadata: Vec<u16>,
  pixels: Vec<u8>,
}

#[wasm_bindgen]
impl DecodeResults {
  #[wasm_bindgen(getter)]
  pub fn frames(&self) -> js_sys::Uint8ClampedArray {
    unsafe {
      js_sys::Uint8ClampedArray::view(&self.pixels)
    }
  }

  #[wasm_bindgen(getter)]
  pub fn metadata(&self) -> js_sys::Uint16Array {
    unsafe {
      js_sys::Uint16Array::view(&self.metadata)
    }
  }
}

#[wasm_bindgen]
pub fn decode(buf: Vec<u8>) -> DecodeResults {
  let mut options = gif::DecodeOptions::new();
  options.set_color_output(gif::ColorOutput::RGBA);
  let mut decoder = options.read_info(buf.as_slice()).unwrap();

  let mut num_frames = 0;

  // [frame1, frame2, frame3, ...]
  // where each frame is (delay, top, left, width, height) but flattened
  let mut metadata = Vec::new();
  let mut pixels = Vec::new();

  while let Some(frame) = decoder.read_next_frame().unwrap() {
    let frame_pixels = frame.buffer.clone().into_owned();
    pixels.extend(&frame_pixels);

    metadata.extend(vec![
      frame.delay,
      frame.top,
      frame.left,
      frame.width,
      frame.height,
    ]);

    num_frames += 1;
  }

  DecodeResults { num_frames, metadata, pixels }
}