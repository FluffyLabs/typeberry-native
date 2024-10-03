use reed_solomon::Error;
use reed_solomon::ReedSolomonDecoder;
use reed_solomon::ReedSolomonEncoder;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys;

#[wasm_bindgen]
pub struct Shard {
    index: u16,
    data: js_sys::Uint8Array,
}

#[wasm_bindgen]
impl Shard {
    #[wasm_bindgen(constructor)]
    pub fn new(index: u16, data: js_sys::Uint8Array) -> Shard {
        Shard { index, data }
    }

    #[wasm_bindgen(getter)]
    pub fn index(&self) -> u16 {
        self.index
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> js_sys::Uint8Array {
        self.data.clone()
    }
}

fn rs_encode(
    original_count: usize,
    recovery_count: usize,
    shard_bytes: usize,
    shards: Vec<js_sys::Uint8Array>,
) -> Result<Vec<js_sys::Uint8Array>, Error> {
    let mut encoder = ReedSolomonEncoder::new(original_count, recovery_count, shard_bytes)?;

    for shard in shards {
        encoder.add_original_shard(shard.to_vec())?;
    }

    let result = encoder.encode()?;

    return result
        .recovery_iter()
        .map(|slice| {
            let array = js_sys::Uint8Array::new_with_length(slice.len() as u32);
            array.copy_from(slice);
            Ok(array)
        })
        .collect();
}

fn rs_decode(
    original_count: usize,
    recovery_count: usize,
    shard_bytes: usize,
    shards: Vec<Shard>,
) -> Result<Vec<Shard>, Error> {
    let mut decoder = ReedSolomonDecoder::new(original_count, recovery_count, shard_bytes)?;

    for shard in shards {
        let idx_usize = usize::from(shard.index);
        if idx_usize < original_count {
            decoder.add_original_shard(idx_usize, shard.data.to_vec())?;
        } else {
            decoder.add_recovery_shard(idx_usize - original_count, shard.data.to_vec())?;
        }
    }

    let decoding_result = decoder.decode()?;

    return decoding_result
        .restored_original_iter()
        .map(|(idx, slice)| {
            let array = js_sys::Uint8Array::new_with_length(slice.len() as u32);
            array.copy_from(slice);
            let shard = Shard::new(idx as u16, array);
            Ok(shard)
        })
        .collect();
}

#[wasm_bindgen]
pub fn encode(
    original_count: usize,
    recovery_count: usize,
    shard_bytes: usize,
    shards: Vec<js_sys::Uint8Array>,
) -> Result<Vec<js_sys::Uint8Array>, String> {
    let result = rs_encode(original_count, recovery_count, shard_bytes, shards);

    result.map_err(|e| e.to_string())
}

#[wasm_bindgen]
pub fn decode(
    original_count: usize,
    recovery_count: usize,
    shard_bytes: usize,
    shards: Vec<Shard>,
) -> Result<Vec<Shard>, String> {
    let result = rs_decode(original_count, recovery_count, shard_bytes, shards);

    result.map_err(|e| e.to_string())
}
