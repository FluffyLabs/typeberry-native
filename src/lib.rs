use reed_solomon::Error;
use reed_solomon::ReedSolomonDecoder;
use reed_solomon::ReedSolomonEncoder;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::js_sys;

/// Collection of shards (either input or output).
///
/// To efficiently pass data between JS and WASM all of the shards
/// are passed as one big vector of bytes.
/// It's assumed that every shard has the same length (`shard_len`).
/// If the shards are NOT passed in the exact order they were created
/// it's possible to pass `indices` array.
/// A value of `indices` array at position `idx` is the shard index
/// that resides at `[ idx * shard_len .. idx * shard_len + shard_len )`
/// in `data` array.
///
/// This collection is only used to get the data from JS or pass the data back.
/// Internally we convert it to [`RsShardsCollection`], which copies
/// the memory to/from WASM.
#[wasm_bindgen]
pub struct ShardsCollection {
    /// Number of shards within the collection.
    pub length: u32,
    /// The length of each shard.
    pub shard_len: u16,
    /// All shards concatenated.
    data: js_sys::Uint8Array,
    /// Optional indices for shards in the collection.
    indices: Option<js_sys::Uint16Array>,
}

#[wasm_bindgen]
impl ShardsCollection {
    #[wasm_bindgen(constructor)]
    pub fn new(
        shard_len: u16,
        data: js_sys::Uint8Array,
        indices: Option<js_sys::Uint16Array>,
    ) -> Self {
        let length = data.length() / shard_len as u32;
        if let Some(ref indices) = indices {
            assert!(
                indices.length() == length,
                "Mismatching indices and data length."
            );
        }

        Self {
            length,
            shard_len,
            indices,
            data,
        }
    }

    /// Extract the `indices` from this shards container.
    ///
    /// Should be called on the JS side to avoid copying.
    /// NOTE that subsequent calls to that method will return `None`.
    #[wasm_bindgen]
    pub fn take_indices(&mut self) -> Option<js_sys::Uint16Array> {
        self.indices.take()
    }

    /// Take the underlying `data` to the JS side.
    ///
    /// NOTE this object is destroyed after the data is consumed,
    /// so make sure to [`take_indices`] first.
    #[wasm_bindgen]
    pub fn take_data(self) -> js_sys::Uint8Array {
        self.data
    }

    // THESE METHODS SHOULD RATHER BE IMPLEMENTED IN JS!
    /*
        #[wasm_bindgen(getter)]
        pub fn len(&self) -> usize {
            self.length as usize
        }

        #[wasm_bindgen(getter)]
        pub fn chunk_at(&self, index: usize) -> js_sys::Uint8Array {
            let begin = index as u32 * self.shard_len as u32;
            let end = begin + self.shard_len as u32;
            self.data.subarray(begin, end)
        }

        #[wasm_bindgen(getter)]
        pub fn chunk_index_at(&self, index: usize) -> u16 {
            self.indices
                .as_ref()
                .map(|v| v.at(index as i32).expect("Out of bounds access to indices."))
                .unwrap_or(index as u16)
        }
    */
}

/// A Rust equivalent of [`ShardsCollection`].
struct RsShardsCollection {
    pub length: usize,
    pub shard_len: u16,
    pub data: Vec<u8>,
    pub indices: Option<Vec<u16>>,
}

impl RsShardsCollection {
    /// Get shard data of chunk at index `index`.
    ///
    /// NOTE that it DOES NOT mean that the shard index necesarrily
    /// matches the `index` value. Make sure to check `chunk_index_at`
    /// to get the `index` of that shard as it was originally passed to `encode`.
    pub fn chunk_at(&self, index: usize) -> &[u8] {
        let begin = index * self.shard_len as usize;
        let end = begin + self.shard_len as usize;

        &self.data[begin..end]
    }

    /// Retrieve the shard index of given chunk.
    ///
    /// This method will default to returning `index`
    /// if the `indices` array is not provided.
    pub fn chunk_index_at(&self, index: usize) -> u16 {
        self.indices
            .as_ref()
            .map(|v| v[index])
            .unwrap_or(index as u16)
    }
}

/// Copy all of the WASM memory to JS.
impl From<RsShardsCollection> for ShardsCollection {
    fn from(value: RsShardsCollection) -> Self {
        let RsShardsCollection {
            length,
            shard_len,
            data,
            indices,
        } = value;

        Self {
            length: length as u32,
            shard_len,
            data: data.as_slice().into(),
            indices: indices.map(|i| i.as_slice().into()),
        }
    }
}

/// Copy all of the JS memory to WASM.
impl From<ShardsCollection> for RsShardsCollection {
    fn from(value: ShardsCollection) -> Self {
        let ShardsCollection {
            length,
            shard_len,
            data,
            indices,
        } = value;

        Self {
            length: length as usize,
            shard_len,
            data: data.to_vec(),
            indices: indices.map(|v| v.to_vec()),
        }
    }
}

fn rs_encode(
    recovery_count: usize,
    shard_bytes: usize, // TODO [ToDr] is this the same as `shards.shard_len`?
    shards: RsShardsCollection,
) -> Result<RsShardsCollection, Error> {
    let mut encoder = ReedSolomonEncoder::new(shards.length, recovery_count, shard_bytes)?;

    for i in 0..shards.length {
        assert!(
            shards.chunk_index_at(i) == i as u16,
            "Input shards must be in order!"
        );
        encoder.add_original_shard(shards.chunk_at(i))?;
    }

    let result = encoder.encode()?;

    let mut data = Vec::with_capacity(recovery_count * shards.shard_len as usize);

    let mut indices = vec![];
    for (idx, chunk) in result.recovery_iter().enumerate() {
        indices.push((shards.length + idx) as u16);
        data.extend(chunk);
    }

    Ok(RsShardsCollection {
        length: recovery_count,
        shard_len: shards.shard_len,
        data,
        indices: Some(indices),
    })
}

fn rs_decode(
    original_count: usize,
    recovery_count: usize,
    shard_bytes: usize,
    shards: RsShardsCollection,
) -> Result<RsShardsCollection, Error> {
    let mut decoder = ReedSolomonDecoder::new(original_count, recovery_count, shard_bytes)?;

    for i in 0..shards.length {
        let idx = shards.chunk_index_at(i) as usize;
        let data = shards.chunk_at(i);
        if idx < original_count {
            decoder.add_original_shard(idx, data)?;
        } else {
            decoder.add_recovery_shard(idx - original_count, data)?;
        }
    }

    let decoding_result = decoder.decode()?;

    let mut indices = vec![];
    let mut data = vec![];
    for (idx, shard) in decoding_result.restored_original_iter() {
        indices.push(idx as u16);
        data.extend(shard);
    }

    Ok(RsShardsCollection {
        length: indices.len(),
        shard_len: shards.shard_len,
        indices: Some(indices),
        data,
    })
}

#[wasm_bindgen]
pub fn encode(
    recovery_count: u16,
    shard_bytes: u16,
    shards: ShardsCollection,
) -> Result<ShardsCollection, String> {
    let result = rs_encode(recovery_count as usize, shard_bytes as usize, shards.into())
        .map_err(|e| e.to_string())?;

    Ok(result.into())
}

#[wasm_bindgen]
pub fn decode(
    original_count: u16,
    recovery_count: u16,
    shard_bytes: u16,
    shards: ShardsCollection,
) -> Result<ShardsCollection, String> {
    let result = rs_decode(
        original_count as usize,
        recovery_count as usize,
        shard_bytes as usize,
        shards.into(),
    )
    .map_err(|e| e.to_string())?;

    Ok(result.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SHARD: usize = 64;

    fn test_data(recovery_count: usize) -> RsShardsCollection {
        let shard_bytes = SHARD;

        let mut data = vec![];
        data.extend([1u8; SHARD]);
        data.extend([2u8; SHARD]);
        data.extend([3u8; SHARD]);

        let shards = RsShardsCollection {
            length: 3,
            shard_len: shard_bytes as u16,
            data,
            indices: None,
        };

        rs_encode(recovery_count, shard_bytes, shards).unwrap()
    }

    #[test]
    fn should_encode_shards() {
        let recovery_count = 5;
        let encoded = test_data(recovery_count);

        assert_eq!(encoded.length, recovery_count);
        assert_eq!(encoded.shard_len, SHARD as u16);
        assert_eq!(encoded.chunk_index_at(0), 3);
        assert_eq!(encoded.chunk_index_at(1), 4);
        assert_eq!(encoded.chunk_index_at(2), 5);
        assert_eq!(encoded.chunk_index_at(3), 6);
        assert_eq!(encoded.chunk_index_at(4), 7);
        assert_eq!(
            encoded.chunk_at(0),
            &[
                156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 156,
                156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 156, 15,
                15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
                15, 15, 15, 15, 15, 15, 15, 15, 15, 15
            ]
        );
        assert_eq!(
            encoded.chunk_at(1),
            &[
                159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 159,
                159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 159, 12,
                12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
                12, 12, 12, 12, 12, 12, 12, 12, 12, 12
            ]
        );
        assert_eq!(
            encoded.chunk_at(2),
            &[
                158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 158,
                158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 158, 13,
                13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
                13, 13, 13, 13, 13, 13, 13, 13, 13, 13
            ]
        );
        assert_eq!(
            encoded.chunk_at(3),
            &[
                157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 157,
                157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 157, 14,
                14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
                14, 14, 14, 14, 14, 14, 14, 14, 14, 14
            ]
        );
        assert_eq!(
            encoded.chunk_at(4),
            &[
                175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 175,
                175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 175, 8,
                8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8
            ]
        );
    }

    #[test]
    fn should_decode_shards() {
        let recovery_count = 6;
        let encoded = test_data(recovery_count);

        let to_decode = RsShardsCollection {
            length: 3,
            shard_len: encoded.shard_len,
            data: {
                let mut data = vec![];
                data.extend(encoded.chunk_at(0));
                data.extend(&[2u8; SHARD]);
                data.extend(encoded.chunk_at(4));
                data
            },
            indices: vec![encoded.chunk_index_at(0), 1, encoded.chunk_index_at(4)].into(),
        };

        let decoded = rs_decode(3, recovery_count, SHARD, to_decode).unwrap();

        assert_eq!(decoded.length, 2);
        assert_eq!(decoded.shard_len, encoded.shard_len);
        assert_eq!(decoded.chunk_index_at(0), 0u16);
        assert_eq!(decoded.chunk_index_at(1), 2u16);
        assert_eq!(decoded.chunk_at(0), &[1u8; SHARD]);
        assert_eq!(decoded.chunk_at(1), &[3u8; SHARD]);
    }
}
