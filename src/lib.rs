mod utils;

extern crate gif;
extern crate wasm_bindgen;

use js_sys::{Uint8Array, Uint16Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace=console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub struct DecodedGif {
  pub num_frames: usize,
  metadata: Vec<u16>,
  pixels: Vec<u8>,
}

#[wasm_bindgen]
impl DecodedGif {
  #[wasm_bindgen(getter)]
  pub fn frames(&self) -> Uint8Array {
    unsafe { Uint8Array::view(&self.pixels) }
  }

  #[wasm_bindgen(getter)]
  pub fn metadata(&self) -> js_sys::Uint16Array {
    unsafe { Uint16Array::view(&self.metadata) }
  }
}

#[wasm_bindgen]
pub fn decode(buf: Vec<u8>) -> DecodedGif {
  let mut options = gif::DecodeOptions::new();
  options.set_color_output(gif::ColorOutput::RGBA);
  let mut decoder = options.read_info(buf.as_slice()).unwrap();

  // (delay, top, left, width, height)
  let mut metadata = Vec::new();
  let mut pixels = Vec::new();
  let mut num_frames = 0;

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

  DecodedGif {
    num_frames,
    metadata,
    pixels,
  }
}

// Going to have to go with this approach until lifetimes are supported
#[wasm_bindgen]
pub struct Encoder {
  pub width: u16,
  pub height: u16,
  data: Vec<u8>,
}

#[wasm_bindgen]
impl Encoder {
  #[wasm_bindgen(constructor)]
  pub fn new(width: u16, height: u16) -> Self {
    Self {
      width,
      height,
      data: Vec::new(),
    }
  }

  pub fn add_frame(&mut self, pixels: Vec<u8>) -> () {
    self.data.extend(pixels);
  }

  pub fn finish(&mut self) -> Uint8Array {
    let mut data = Vec::new();

    {
      let mut encoder = gif::Encoder::new(&mut data, self.width, self.height, &[]).unwrap();
      encoder.set_repeat(gif::Repeat::Infinite).unwrap();
      
      // assume rgba
      let sz = self.width * self.height * 4;
      for frame in self.data.chunks_mut(sz as usize) {
        let mut frame = gif::Frame::from_rgba_speed(self.width, self.height, frame, 10);
        frame.dispose = gif::DisposalMethod::Previous;
        frame.delay = 2;
        encoder.write_frame(&frame).unwrap();
      }
    }

    unsafe { Uint8Array::view(&data) }
  }
}

// This works, but requires JS to pre-allocate some Uint8Array with enough space to contain the output
// and I don't know how to get this size. Also, runs into some MLE issues.

// Hack from #2642 to obtain a mutable immutable (lol) view of a JS array
// #[wasm_bindgen(inline_js = "export const identity = x => x")]
// extern "C" {
//   #[wasm_bindgen(js_name=identity)]
//   fn safe_view(slice: &[u8]) -> Uint8ClampedArray;
// }

// struct WritableUint8ClampedArray {
//   ptr: u32,
//   data: Uint8ClampedArray,
// }

// impl WritableUint8ClampedArray {
//   pub fn new(data: Uint8ClampedArray) -> Self {
//     Self {
//       data,
//       ptr: 0,
//     }
//   }
// }

// impl std::io::Write for WritableUint8ClampedArray {
//   fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//     // can overflow
//     let len = buf.len() as u32;
//     let buffer = Uint8ClampedArray::new_with_length(len);
//     buffer.copy_from(buf);
//     self.data.set(&buffer, self.ptr);
//     log(&format!("paul pogba {} {}", self.ptr, self.data.get_index(self.ptr)));
//     self.ptr += len;
//     Ok(buf.len())
//   }

//   fn flush(&mut self) -> std::io::Result<()> {
//     Ok(())
//   }
// }

// #[wasm_bindgen]
// pub fn create_encoder(width: u16, height: u16, data: &[u8]) -> JsValue {
//   let data = safe_view(data);
//   let data = WritableUint8ClampedArray::new(data);
//   let mut encoder = gif::Encoder::new(data, width, height, &[]).unwrap();
//   encoder.set_repeat(gif::Repeat::Infinite).unwrap();

//   let add_frame_cb = Closure::new(
//     Box::new(move |pixels: Vec<u8>| {
//       let mut pixels = pixels.clone();
//       let mut frame = gif::Frame::from_rgba_speed(width, height, &mut pixels, 10);
//       frame.dispose = gif::DisposalMethod::Previous;
//       frame.delay = 2;
//       encoder.write_frame(&frame).unwrap();
//     }) as Box<dyn FnMut(Vec<u8>) -> ()>
//   );

//   let add_frame = add_frame_cb.as_ref().clone();
//   add_frame_cb.forget();

//   add_frame
// }
