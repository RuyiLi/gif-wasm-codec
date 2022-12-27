/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} buf
* @returns {DecodedGif}
*/
export function decode(buf: Uint8Array): DecodedGif;
/**
*/
export class DecodedGif {
  free(): void;
/**
*/
  readonly frames: Uint8ClampedArray;
/**
*/
  readonly metadata: Uint16Array;
/**
*/
  num_frames: number;
}
/**
*/
export class Encoder {
  free(): void;
/**
* @param {number} width
* @param {number} height
*/
  constructor(width: number, height: number);
/**
* @param {Uint8Array} pixels
*/
  add_frame(pixels: Uint8Array): void;
/**
* @returns {Uint8ClampedArray}
*/
  finish(): Uint8ClampedArray;
/**
*/
  height: number;
/**
*/
  width: number;
}
