//! Tooling crate utilities for simulation validation and helper logic.

pub const MAX_TICK_RATE: u32 = 3600;

pub fn sanitize_tick_rate(requested: u32) -> u32 {
    requested.clamp(1, MAX_TICK_RATE)
}

pub fn is_valid_tick_rate(requested: u32) -> bool {
    (1..=MAX_TICK_RATE).contains(&requested)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_tick_rate_bounds() {
        assert_eq!(sanitize_tick_rate(0), 1);
        assert_eq!(sanitize_tick_rate(MAX_TICK_RATE + 1), MAX_TICK_RATE);
        assert_eq!(sanitize_tick_rate(60), 60);
    }

    #[test]
    fn validates_supported_tick_rates() {
        assert!(!is_valid_tick_rate(0));
        assert!(is_valid_tick_rate(120));
        assert!(!is_valid_tick_rate(MAX_TICK_RATE + 1));
    }
}
