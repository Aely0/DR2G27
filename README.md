# DR2G27
Small program that maps DiRT Rally 2.0 in-game RPM data to Logitech G27 Racing Wheel's RPM LED bar.

### LED Mapping
Active LED states are mapped linearly to RPM range `[(max - (max - idle) / 2)..max]`.

### Requirements
Requires DiRT Rally 2.0 telemetry option to be enabled.

It can be enabled in the following file:\
`C:\Users\<UserName>\Documents\My Games\DiRT Rally 2.0\hardwaresettings\hardware_settings_config.xml`

By setting udp value to:\
`<udp enabled="true" extradata="3" ip="127.0.0.1" port="20777" delay="1" />`.

### Download
Executable can be found [here](https://github.com/Andris0/DR2G27/releases/download/v1.0.0/dr2g27.exe).

Compiled with: `Rust 1.40.0`\
For target: `x86_64-pc-windows-msvc`\
MD5: `5280a4fe44337dc93ef4750ff0de9f40`
