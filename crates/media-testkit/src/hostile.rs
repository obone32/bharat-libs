//! Deterministic hostile-input hammer, from [`bharat_testkit::hostile`]: [`hammer`] runs a parser on
//! a valid seed, every truncation of it and seeded damage of it, one case after another on a worker
//! thread with a time limit for each case. [`Rng`], [`mutations`] and the other names here are
//! `bharat_testkit`'s.

#[cfg(not(target_arch = "wasm32"))]
pub use bharat_testkit::hostile::{HammerConfig, HammerError};
pub use bharat_testkit::hostile::{MAX_CASES, Mutation, Mutations, Rng, mutations};

/// The variable that multiplies [`hammer`]'s iterations.
#[cfg(not(target_arch = "wasm32"))]
const MULTIPLIER_VARIABLE: &str = "BHARAT_MEDIA_HOSTILE_MULT";

/// Run `run` over `seed`, every truncation of it, then `iters` random mutations (bit flips, random
/// bytes, 1/2/4/8-byte extreme-value overwrites in both endiannesses, deletions, truncation). The
/// cases run on a worker thread with a limit of 5 s each, which `BHARAT_TEST_TIMEOUT_SECS` replaces.
/// Panics on the first case that panics or does not finish, naming `name`, the case, its length and
/// the panic's message.
///
/// `BHARAT_MEDIA_HOSTILE_MULT=10` runs ten times the iterations (soak runs). A value that is not a
/// whole number above zero panics, naming the variable, before `run` is called.
#[cfg(not(target_arch = "wasm32"))]
#[track_caller]
pub fn hammer(name: &str, seed: &[u8], iters: usize, run: impl Fn(&[u8]) + Send + Sync + 'static) {
    bharat_testkit::hostile::hammer(&HammerConfig::new(name, iters).with_multiplier_variable(MULTIPLIER_VARIABLE), seed, run);
}
