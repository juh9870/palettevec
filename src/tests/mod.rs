pub(crate) mod cases;
pub(crate) mod implementations;

/// Controls the amount of iterations the tests do.
/// 0.0 means 0 iterations, 1.0 means base line,
/// 2.0 means twice the base line etc.
const TEST_EXTENSIVENESS: f64 = 1.0;
