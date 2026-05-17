//! In-memory per-IP rate limiter for the login endpoint. Single-instance
//! backend behind nginx; if this ever needs to scale horizontally, swap
//! this for a Redis-backed limiter (interface is intentionally narrow).

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Sliding-window counter. Records each attempt's timestamp; on every call
/// it prunes timestamps older than `WINDOW`, then accepts/rejects based on
/// the remaining count.
pub struct LoginThrottle {
    window: Duration,
    max_attempts: usize,
    state: Mutex<HashMap<IpAddr, Vec<Instant>>>,
}

impl LoginThrottle {
    /// Default tuning: 5 attempts per minute per IP. After the 5th try the
    /// 6th call is rejected with retry-after = (window - oldest_age).
    pub fn new() -> Self {
        Self::with_limits(Duration::from_secs(60), 5)
    }

    pub fn with_limits(window: Duration, max_attempts: usize) -> Self {
        Self {
            window,
            max_attempts,
            state: Mutex::new(HashMap::new()),
        }
    }

    /// Records the attempt and returns Ok(()) if under the limit, or
    /// Err(retry_after) if the IP must back off. Records the attempt
    /// regardless of credential outcome — that way credential stuffing
    /// can't bypass the limit by guessing different usernames.
    pub fn check_and_record(&self, ip: IpAddr) -> Result<(), Duration> {
        let now = Instant::now();
        let mut guard = self
            .state
            .lock()
            .expect("login throttle mutex poisoned");

        let attempts = guard.entry(ip).or_default();

        // Prune stamps outside the window.
        attempts.retain(|t| now.duration_since(*t) < self.window);

        if attempts.len() >= self.max_attempts {
            let oldest = attempts
                .first()
                .copied()
                .unwrap_or(now);
            let retry_after = self.window.saturating_sub(now.duration_since(oldest));
            // Don't push the current attempt — that would keep extending
            // the lockout for every retry. The window is purely sliding
            // over recorded attempts.
            return Err(retry_after);
        }

        attempts.push(now);
        Ok(())
    }

    /// Background sweeper to prevent map growth for inactive IPs.
    /// Cheap to call periodically; safe to skip if you forget.
    pub fn sweep(&self) {
        let now = Instant::now();
        let window = self.window;
        let mut guard = match self.state.lock() {
            Ok(g) => g,
            Err(_) => return,
        };
        guard.retain(|_, attempts| {
            attempts.retain(|t| now.duration_since(*t) < window);
            !attempts.is_empty()
        });
    }
}

impl Default for LoginThrottle {
    fn default() -> Self {
        Self::new()
    }
}
