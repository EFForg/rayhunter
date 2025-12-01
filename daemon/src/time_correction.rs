use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Stores a time offset that corrects the system clock without modifying it.
///
/// This is used on devices where the system clock is incorrect (e.g., devices
/// without cellular connectivity that can't sync via NITZ). Instead of trying
/// to modify the system clock (which may require root permissions and fail),
/// we store an offset and apply it to all timestamps that Rayhunter produces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeCorrection {
    /// The offset to add to system time to get correct time (in seconds)
    /// Positive means system clock is behind, negative means ahead
    pub offset_seconds: i64,

    /// When this offset was last updated (using corrected time)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<DateTime<Utc>>,
}

impl Default for TimeCorrection {
    fn default() -> Self {
        Self {
            offset_seconds: 0,
            last_updated: None,
        }
    }
}

impl TimeCorrection {
    /// Create a new time correction with zero offset
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the time correction from a browser/client timestamp
    ///
    /// # Arguments
    /// * `browser_timestamp` - Unix timestamp in milliseconds from the browser
    pub fn set_from_browser(&mut self, browser_timestamp_ms: i64) {
        let browser_time = match Utc.timestamp_millis_opt(browser_timestamp_ms).single() {
            Some(dt) => dt,
            None => {
                log::warn!(
                    "Invalid browser timestamp received: {}, falling back to current system time",
                    browser_timestamp_ms
                );
                Utc::now()
            }
        };

        let system_now = Utc::now();
        let offset_duration = browser_time.signed_duration_since(system_now);
        self.offset_seconds = offset_duration.num_seconds();
        self.last_updated = Some(browser_time);
    }

    /// Get the current offset in seconds
    pub fn offset_seconds(&self) -> i64 {
        self.offset_seconds
    }
}

/// Shared time correction state
pub type TimeCorrectionState = Arc<RwLock<TimeCorrection>>;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_default_correction() {
        let tc = TimeCorrection::new();
        assert_eq!(tc.offset_seconds(), 0);
        assert!(tc.last_updated.is_none());
    }

    #[test]
    fn test_set_from_browser() {
        let mut tc = TimeCorrection::new();

        // Simulate browser time being 1 hour ahead
        let system_now = Utc::now();
        let browser_time = system_now + Duration::hours(1);
        let browser_timestamp_ms = browser_time.timestamp_millis();

        tc.set_from_browser(browser_timestamp_ms);

        // Offset should be approximately 1 hour (3600 seconds)
        // We allow a small margin for test execution time
        let offset = tc.offset_seconds();
        assert!(offset > 3595 && offset < 3605, "Offset was {}", offset);
    }

    #[test]
    fn test_invalid_browser_timestamp() {
        let mut tc = TimeCorrection::new();

        // Use an invalid timestamp that will cause timestamp_millis_opt to return None
        // i64::MAX is well beyond the valid range for Unix timestamps in milliseconds
        let invalid_timestamp_ms = i64::MAX;

        tc.set_from_browser(invalid_timestamp_ms);

        // When an invalid timestamp is provided, it should fall back to system time
        // This means the offset should be approximately 0 (within a small margin for execution time)
        let offset = tc.offset_seconds();
        assert!(
            offset.abs() < 5,
            "Expected offset near 0 for invalid timestamp, got {}",
            offset
        );

        // The last_updated should still be set (to the fallback system time)
        assert!(tc.last_updated.is_some());
    }
}
