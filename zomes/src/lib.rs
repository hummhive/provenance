pub enum TestWasm {
    Crypto,
}

impl From<TestWasm> for String {
    fn from(test_wasm: TestWasm) -> Self {
        Self::from(match test_wasm {
            TestWasm::Crypto => "crypto",
        })
    }
}

impl From<TestWasm> for Vec<u8> {
    fn from(test_wasm: TestWasm) -> Self {
        match test_wasm {
            TestWasm::Crypto => include_bytes!(concat!(
                env!("OUT_DIR"),
                "/wasm32-unknown-unknown/release/humm_zome_crypto.wasm"
            ))
            .to_vec(),
        }
    }
}
