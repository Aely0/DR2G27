fn main() {
    #[cfg(not(debug_assertions))]
    {
        use std::path::Path;
        use windows_exe_info::{icon::icon_ico, versioninfo::VersionInfo};

        icon_ico(Path::new("assets/dr2g27.ico"));

        let mut version_info = VersionInfo::from_cargo_env();

        if let Some(file_info) = version_info.file_info.first_mut() {
            file_info.file_description = "Dirt Rally 2.0 G27 LED RPM".into();
            file_info.original_filename = "DR2G27".into();
            file_info.product_name = "DR2G27".into();
        }

        version_info.link().expect("win_info");
    }
}
