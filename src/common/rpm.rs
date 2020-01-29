use std::convert::TryFrom;

#[derive(Default)]
pub struct RPM {
    current: f32,
    max: f32,
    idle: f32,
    staleness: u8,
}

impl RPM {
    const STALENESS_THRESHOLD: u8 = 5;

    pub fn new() -> Self {
        RPM {
            ..Default::default()
        }
    }

    fn f32_from_byte_slice(slice: &[u8]) -> f32 {
        f32::from_le_bytes(<[u8; 4]>::try_from(slice).expect("bytes_to_f32"))
    }

    fn parse_data(data: &[u8]) -> (f32, f32, f32) {
        (
            Self::f32_from_byte_slice(&data[148..152]),
            Self::f32_from_byte_slice(&data[252..256]),
            Self::f32_from_byte_slice(&data[256..260]),
        )
    }

    fn increment_staleness(&mut self) {
        if self.staleness < Self::STALENESS_THRESHOLD {
            self.staleness += 1;
        }
    }

    fn reset_staleness(&mut self) {
        if self.staleness != 0 {
            self.staleness = 0;
        }
    }

    pub fn is_stale(&self) -> bool {
        self.staleness >= Self::STALENESS_THRESHOLD
    }

    pub fn state(&self) -> (f32, f32, f32) {
        (self.current, self.max, self.idle)
    }

    pub fn update(&mut self, data: &[u8]) {
        let parsed_data = Self::parse_data(data);
        if (self.current, self.max, self.idle) == parsed_data {
            self.increment_staleness();
        } else {
            self.reset_staleness();
            self.current = parsed_data.0;
            self.max = parsed_data.1;
            self.idle = parsed_data.2;
        }
    }
}
