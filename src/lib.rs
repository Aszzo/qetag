#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use base64_url::encode;
use futures::prelude::*;
use napi::bindgen_prelude::*;
use napi::*;
use sha1::{Digest, Sha1};

const BLOCK_BITS: u8 = 22;
const BLOCK_SIZE: usize = 1 << BLOCK_BITS;

#[napi]
async fn get_etag(path: String) -> Result<String> {
  tokio::fs::read(path)
    .map(|r| match r {
      Ok(content) => Ok(calc_etag(content)),
      Err(e) => Err(Error::new(
        Status::GenericFailure,
        format!("failed to read file, {}", e),
      )),
    })
    .await
}

fn calc_sha1(buf: &Vec<u8>) -> Vec<u8> {
  let mut hasher = Sha1::new();

  hasher.update(buf);

  let result = hasher.finalize();

  result[..].to_vec()
}

fn calc_etag(buffer: Vec<u8>) -> String {
  let buf_len = buffer.len();
  let block_count = (buf_len as f64 / BLOCK_SIZE as f64).ceil();

  let mut hash_arr = Vec::new();

  // 小于4M
  if block_count as usize <= 1 {
    let sha1 = calc_sha1(&buffer);

    hash_arr.push(0x16);
    hash_arr.extend_from_slice(&sha1);
  } else {
    let mut sha1_vec = vec![];
    for i in 0..block_count as usize {
      let start = BLOCK_SIZE * i;
      let end = {
        if BLOCK_SIZE * (i + 1) >= buf_len {
          buf_len
        } else {
          BLOCK_SIZE * (i + 1)
        }
      };

      let sha1 = calc_sha1(&buffer[start..end].to_vec());
      sha1_vec.extend_from_slice(&sha1);
    }

    let sha1_vec_sha1 = calc_sha1(&sha1_vec);

    hash_arr.push(0x96);
    hash_arr.extend_from_slice(&sha1_vec_sha1);
  }

  let result = encode(&hash_arr);

  result
}
