//! The crate's integration tests, linked into one test binary.

mod animation;
mod atn_contract;
mod atn_v12;
mod builder;
mod corpus;
mod corpus_atn;
mod layer_block_order;
mod malformed;
mod merged_recovery;
mod proptests;
mod roundtrip;
mod swatch_malformed;
mod tree;

/// A directory of the real-file corpus (feature `corpus`): under `BHARAT_PSD_CORPUS` when set (a
/// PhotoCraft checkout's `corpus` directory), else `corpus/` at the workspace root.
#[cfg(feature = "corpus")]
fn corpus_dir(sub: &str) -> std::path::PathBuf {
    let root = std::env::var_os("BHARAT_PSD_CORPUS")
        .map_or_else(|| std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../corpus"), std::path::PathBuf::from);
    root.join(sub)
}

/// The self dev-dependency builds these tests with `test-support`, the feature that gates
/// `bharat_psd::fixtures` and `bharat_psd::testgen`.
#[test]
fn tests_build_with_the_test_support_feature() {
    const { assert!(cfg!(feature = "test-support")) };
    let file = bharat_psd::testgen::small(bharat_psd::Version::Psd, bharat_psd::Compression::Raw);
    assert_eq!(file.header.version, bharat_psd::Version::Psd);
    let _sample: fn(u32, u32, u32, bool) -> u8 = bharat_psd::fixtures::sample;
}
