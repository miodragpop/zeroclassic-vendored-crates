#![allow(dead_code)]
use atomic_shim::AtomicU64;
use std::{
    sync::{atomic::Ordering, Arc},
    time::Duration,
};

/// Type which can be converted into a nanosecond representation.
///
/// This allows users of [`Mock`] to increment/decrement the time both with raw
/// integer values and the more convenient [`Duration`] type.
pub trait IntoNanoseconds {
    fn into_nanos(self) -> u64;
}

impl IntoNanoseconds for u64 {
    fn into_nanos(self) -> u64 {
        self
    }
}

impl IntoNanoseconds for Duration {
    fn into_nanos(self) -> u64 {
        self.as_nanos() as u64
    }
}

/// Controllable time source for use in tests.
///
/// A mocked clock allows the caller to adjust the given time backwards and forwards by whatever
/// amount they choose.  While [`Clock`](crate::Clock) promises monotonic values for normal readings,
/// when running in mocked mode, these guarantees do not apply: the given `Clock`/`Mock` pair are
/// directly coupled.
///
/// This can be useful for not only testing code that depends on the passage of time, but also for
/// testing that code can handle large shifts in time.
#[derive(Debug, Clone)]
pub struct Mock {
    offset: Arc<AtomicU64>,
}

impl Mock {
    pub(crate) fn new() -> Self {
        Self {
            offset: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Increments the time by the given amount.
    pub fn increment<N: IntoNanoseconds>(&self, amount: N) {
        self.offset
            .fetch_add(amount.into_nanos(), Ordering::Release);
    }

    /// Decrements the time by the given amount.
    pub fn decrement<N: IntoNanoseconds>(&self, amount: N) {
        self.offset
            .fetch_sub(amount.into_nanos(), Ordering::Release);
    }

    /// Gets the current value of this `Mock`.
    pub fn value(&self) -> u64 {
        self.offset.load(Ordering::Acquire)
    }
}
