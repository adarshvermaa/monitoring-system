use std::time::Duration;

/// Exponential backoff retry policy with jitter
pub struct RetryPolicy {
    max_retries: usize,
    current_attempt: usize,
    initial_delay: Duration,
    max_delay: Duration,
}

impl RetryPolicy {
    pub fn new(max_retries: usize, initial_delay: Duration, max_delay: Duration) -> Self {
        Self {
            max_retries,
            current_attempt: 0,
            initial_delay,
            max_delay,
        }
    }

    /// Get the delay for the next retry attempt
    /// Returns None if max retries exceeded
    pub fn next_delay(&mut self) -> Option<Duration> {
        if self.current_attempt >= self.max_retries {
            return None;
        }

        self.current_attempt += 1;

        // Exponential backoff: delay = initial * 2^attempt
        let delay_secs = self.initial_delay.as_secs() * (2_u64.pow(self.current_attempt as u32 - 1));
        let delay = Duration::from_secs(delay_secs.min(self.max_delay.as_secs()));

        // Add jitter (±20%)
        let jitter = (delay.as_millis() as f64 * 0.2 * (rand::random::<f64>() - 0.5)) as u64;
        let delay_with_jitter = Duration::from_millis(delay.as_millis() as u64 + jitter);

        Some(delay_with_jitter)
    }

    /// Reset retry counter
    pub fn reset(&mut self) {
        self.current_attempt = 0;
    }

    /// Get current attempt number
    pub fn current_attempt(&self) -> usize {
        self.current_attempt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_backoff() {
        let mut policy = RetryPolicy::new(3, Duration::from_secs(1), Duration::from_secs(60));

        let delay1 = policy.next_delay();
        assert!(delay1.is_some());
        assert!(delay1.unwrap().as_secs() <= 2);

        let delay2 = policy.next_delay();
        assert!(delay2.is_some());
        assert!(delay2.unwrap().as_secs() <= 4);

        let delay3 = policy.next_delay();
        assert!(delay3.is_some());

        let delay4 = policy.next_delay();
        assert!(delay4.is_none()); // Max retries exceeded
    }

    #[test]
    fn test_reset() {
        let mut policy = RetryPolicy::new(3, Duration::from_secs(1), Duration::from_secs(60));

        policy.next_delay();
        policy.next_delay();
        assert_eq!(policy.current_attempt(), 2);

        policy.reset();
        assert_eq!(policy.current_attempt(), 0);
    }
}
