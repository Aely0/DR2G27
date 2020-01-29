use winres::WindowsResource;

fn main() {
    WindowsResource::new()
        .set("FileDescription", "Dirt Rally 2.0 G27 LED RPM")
        .set("ProductVersion", "1.0.0")
        .set("OriginalFilename", "DR2G27")
        .set("ProductName", "DR2G27")
        .set_icon("assets/dr2g27.ico")
        .compile()
        .expect("winres");
}
