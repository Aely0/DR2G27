use crate::common::rpm::RPM;
use crate::common::util::DR2G27Result;
use core::u8::{MAX, MIN};
use hidapi::HidDevice;

pub struct LEDS {
    device: HidDevice,
    rpm: RPM,
    state: u8,
}

impl LEDS {
    pub fn new(device: HidDevice) -> Self {
        LEDS {
            device: device,
            rpm: RPM::new(),
            state: 0,
        }
    }

    const fn led_state_payload(state: u8) -> [u8; 8] {
        [0x00, 0xF8, 0x12, state, 0x00, 0x00, 0x00, 0x01]
    }

    fn percentage_to_led_state(percentage: u8) -> u8 {
        match percentage {
            MIN..=20 => 1,
            21..=40 => 3,
            41..=60 => 7,
            61..=80 => 15,
            81..=MAX => 31,
        }
    }

    fn new_led_state(&self) -> u8 {
        let (rpm_current, rpm_max, rpm_idle) = self.rpm.state();
        match rpm_max - (rpm_max - rpm_idle) / 2_f32 {
            range_start if rpm_current < range_start || range_start as u8 == 0 => 0,
            range_start => {
                let active_range = rpm_max - range_start;
                let current_in_range = rpm_current - range_start;
                let percentage = current_in_range / active_range * 100_f32;
                Self::percentage_to_led_state(percentage as u8)
            }
        }
    }

    fn update_device_and_state(&mut self, new_state: u8) -> DR2G27Result {
        self.device.write(&Self::led_state_payload(new_state))?;
        self.state = new_state;

        Ok(())
    }

    pub fn update(&mut self, data: &[u8]) -> DR2G27Result {
        self.rpm.update(data);

        if !self.rpm.is_stale() {
            let new_state = self.new_led_state();
            if new_state != self.state {
                self.update_device_and_state(new_state)?;
            }
        } else if self.state != 0 {
            self.update_device_and_state(0)?;
        }

        Ok(())
    }
}
