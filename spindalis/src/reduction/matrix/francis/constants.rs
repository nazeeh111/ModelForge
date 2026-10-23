// Tuning constants used by the verification fixtures and deflation bookkeeping.
pub const MAX_ITERS: usize = 100;
pub const TOLERANCE: f64 = 1e-4;
pub const ABSOLUTE_CAP: f64 = 1e-3;
pub const EPSILON: f64 = 1e-21;

pub const EXCEPTION_SHIFT_OFFSET: usize = 8;
pub const EXCEPTION_SHIFT_PERIOD: usize = 12;

// Used in interface.rs as default values for the Francis LQ decomp assuming fully packed matrices
pub const DEFAULT_MAX_ITERS: usize = 100;
pub const DEFAULT_TOLERANCE: f64 = 1e-10;
pub const DEFAULT_ABSOLUTE: f64 = 1e-12;
