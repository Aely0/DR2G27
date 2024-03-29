use dr2g27::common::{
    leds::LEDS,
    util::{DR2G27Result, G27_PID, G27_VID},
};
use hidapi::{HidApi, HidDevice};
use std::{net::UdpSocket, thread::sleep, time::Duration};

// Telemetry config "hardware_settings_config.xml"
// <udp enabled="true" extradata="3" ip="127.0.0.1" port="20777" delay="1" />

fn read_telemetry_and_update(device: HidDevice) -> DR2G27Result {
    let socket = UdpSocket::bind("127.0.0.1:20777")?;
    let mut leds = LEDS::new(device);
    let mut data = [0; 264];

    loop {
        socket.recv(&mut data)?;
        leds.update(&data)?;
    }
}

fn device_connected(hid: &HidApi) -> bool {
    for device in hid.device_list() {
        if device.product_id() == G27_PID && device.vendor_id() == G27_VID {
            return true;
        }
    }

    false
}

fn connect_and_bridge() -> DR2G27Result {
    println!("# Looking for G27");
    let mut hid = HidApi::new()?;

    loop {
        if device_connected(&hid) {
            if let Ok(device) = hid.open(G27_VID, G27_PID) {
                println!("# G27 connected");
                read_telemetry_and_update(device)?;
            }
        }

        sleep(Duration::from_secs(1));
        hid.refresh_devices()?;
    }
}

fn main() {
    loop {
        if let Err(error) = connect_and_bridge() {
            println!("# {:?}", error);
        }

        sleep(Duration::from_secs(1));
    }
}

#[test]
fn test_device_leds() -> DR2G27Result {
    let device = HidApi::new()?.open(G27_VID, G27_PID)?;

    for state in vec![0, 1, 3, 7, 15, 31] {
        device.write(&[0x00, 0xF8, 0x12, state, 0x00, 0x00, 0x00, 0x01])?;
        sleep(Duration::from_millis(200));
    }

    sleep(Duration::from_secs(1));

    for state in vec![31, 15, 7, 3, 1, 0] {
        device.write(&[0x00, 0xF8, 0x12, state, 0x00, 0x00, 0x00, 0x01])?;
        sleep(Duration::from_millis(200));
    }

    Ok(())
}
