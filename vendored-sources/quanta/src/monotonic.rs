#[cfg(any(target_os = "macos", target_os = "ios"))]
use mach::mach_time::{mach_continuous_time, mach_timebase_info};

#[derive(Debug, Clone)]
pub struct Monotonic {
    factor: u64,
}

#[cfg(all(
    not(target_os = "macos"),
    not(target_os = "ios"),
    not(target_os = "windows")
))]
impl Monotonic {
    pub fn new() -> Monotonic {
        Monotonic { factor: 0 }
    }
}

#[cfg(all(
    not(target_os = "macos"),
    not(target_os = "ios"),
    not(target_os = "windows")
))]
impl Monotonic {
    pub fn now(&self) -> u64 {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        unsafe {
            libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts);
        }
        (ts.tv_sec as u64) * 1_000_000_000 + (ts.tv_nsec as u64)
    }
}

#[cfg(target_os = "windows")]
impl Monotonic {
    pub fn new() -> Monotonic {
        use std::mem;
        use winapi::um::profileapi;

        let denom = unsafe {
            let mut freq = mem::zeroed();
            if profileapi::QueryPerformanceFrequency(&mut freq) == 0 {
                unreachable!(
                    "QueryPerformanceFrequency on Windows XP or later should never return zero!"
                );
            }
            *freq.QuadPart() as u64
        };

        Monotonic {
            factor: 1_000_000_000 / denom,
        }
    }
}

#[cfg(target_os = "windows")]
impl Monotonic {
    pub fn now(&self) -> u64 {
        use std::mem;
        use winapi::um::profileapi;

        let raw = unsafe {
            let mut count = mem::zeroed();
            if profileapi::QueryPerformanceCounter(&mut count) == 0 {
                unreachable!(
                    "QueryPerformanceCounter on Windows XP or later should never return zero!"
                );
            }
            *count.QuadPart() as u64
        };
        raw * self.factor
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
impl Monotonic {
    pub fn new() -> Monotonic {
        let mut info = mach_timebase_info { numer: 0, denom: 0 };
        unsafe {
            mach_timebase_info(&mut info);
        }

        let factor = u64::from(info.numer) / u64::from(info.denom);
        Monotonic { factor }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
impl Monotonic {
    pub fn now(&self) -> u64 {
        let raw = unsafe { mach_continuous_time() };
        raw * self.factor
    }
}

impl Default for Monotonic {
    fn default() -> Self {
        Self::new()
    }
}
