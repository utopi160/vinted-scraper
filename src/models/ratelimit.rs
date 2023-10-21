use std::time::{Duration, Instant};

use tokio::sync::RwLock;
use super::ratelimite_parameters::RatelimiteParameters;

pub struct Ratelimiter {
    max_requests: u32,
    duration: Duration,
    current: RwLock<RatelimiteParameters>
}

impl Ratelimiter {
    pub fn new(max_requests: u32, duration: Duration) -> Self {
        return Ratelimiter {
            max_requests,
            duration,

            current: RatelimiteParameters {
                requests: 0,
                last_peak: Instant::now()
            }.into()
        };
    }

    pub async fn try_execute(&self) -> bool {
        let mut current = self.current.write().await;
        let instant = Instant::now();

        if (instant - current.last_peak) >= self.duration {
            current.requests = 1;
            current.last_peak = instant;
            return true; 
        } else if current.requests + 1 < self.max_requests {
            current.requests += 1;
            return true; 
        } else {
            return false;   
        }
    }
}