#![no_main]

use libfuzzer_sys::fuzz_target;

// The doc.dat object stream and the model built from it, wrapped in a stored archive entry so
// mutations reach the tagged grammar instead of the archive's checksum.
fuzz_target!(|stream: &[u8]| {
    let _ = bharat_affinity::stream::parse(stream);
    let file = bharat_affinity::synth::container(&[("doc.dat", stream, bharat_affinity::synth::Method::Stored)], None);
    let _ = bharat_affinity::read(&file, bharat_affinity::Limits { max_entry: 16 << 20, max_total: 64 << 20, max_pixel_bytes: 256 << 20 });
});
