use std::time::Instant;

#[derive(Clone, Copy)]
pub struct RatelimiteParameters {
    pub requests: u32,
    pub last_peak: Instant 
}