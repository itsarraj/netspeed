//! Pure throughput arithmetic — bytes transferred over a measured
//! duration, converted to Mbps. No network access here; `main.rs` is
//! the only part that makes real requests.

use std::time::Duration;

/// Megabits per second: bytes → bits (×8), divided by elapsed seconds,
/// divided by 1,000,000 (decimal mega, matching how ISPs and every
/// mainstream speed test report it — not `1024*1024`).
pub fn mbps(bytes: u64, elapsed: Duration) -> f64 {
    let secs = elapsed.as_secs_f64();
    if secs <= 0.0 {
        return 0.0;
    }
    (bytes as f64 * 8.0) / secs / 1_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_megabyte_in_one_second_is_eight_mbps() {
        // 1,000,000 bytes * 8 bits / 1 second / 1,000,000 = 8.0 Mbps exactly.
        assert_eq!(mbps(1_000_000, Duration::from_secs(1)), 8.0);
    }

    #[test]
    fn ten_megabytes_in_two_seconds_is_forty_mbps() {
        assert_eq!(mbps(10_000_000, Duration::from_secs(2)), 40.0);
    }

    #[test]
    fn zero_bytes_is_zero_mbps() {
        assert_eq!(mbps(0, Duration::from_secs(1)), 0.0);
    }

    #[test]
    fn zero_elapsed_time_is_zero_not_a_divide_by_zero_or_infinity() {
        let result = mbps(1_000_000, Duration::ZERO);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn sub_second_duration_computes_correctly() {
        // 500,000 bytes = 4,000,000 bits; over 0.5s that's 8,000,000 bits/sec = 8 Mbps.
        assert_eq!(mbps(500_000, Duration::from_millis(500)), 8.0);
    }
}
