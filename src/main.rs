#[cfg(target_os = "windows")]
fn main() {
    let seewo_freeze_ui_bin = include_bytes!("SeewoFreezeUI.exe");
    let mut working_dir = std::env::temp_dir();
    working_dir.push("swfreezehelper");

    if working_dir.exists() {
        println!("删除上次操作的临时文件...");
        std::fs::remove_dir_all(working_dir.clone()).expect("错误：无法删除文件");
    };

    let mut seewo_service_dir: std::path::PathBuf = [r"C:\", "Program Files (x86)", "Seewo", "SeewoService"]
        .iter()
        .collect();

    if !seewo_service_dir.exists() {
        return println!("错误：没有找到希沃管家的安装");
    };

    seewo_service_dir.push({
        let seewo_service_ver_regex = regex::Regex::new(r"^SeewoService_[0-9]+").unwrap();
        let mut seewo_service_ver_dirname: String = String::new();
        for entry in std::fs::read_dir(&(seewo_service_dir)).unwrap() {
            if seewo_service_ver_regex
                .is_match(entry.as_ref().unwrap().file_name().to_str().unwrap())
            {
                seewo_service_ver_dirname = entry.unwrap().file_name().into_string().unwrap();
            }
        }
        seewo_service_ver_dirname
    });

    println!("准备中...");
    {
        let mut seewo_service_freeze_dir = seewo_service_dir.clone();
        seewo_service_freeze_dir.push("SeewoFreeze");
        copy_dir::copy_dir(seewo_service_freeze_dir, working_dir.clone())
            .expect("错误：无法复制文件");
    };
    {
        let mut seewo_service_crashsdk = seewo_service_dir.clone();
        seewo_service_crashsdk.push("CrashSDK.dll");
        let mut working_dir_crashsdk = working_dir.clone();
        working_dir_crashsdk.push("CrashSDK.dll");
        std::fs::copy(seewo_service_crashsdk, working_dir_crashsdk)
            .expect("错误：无法复制文件 CrashSDK.dll");
    };
    let mut seewo_service_freezeui = working_dir.clone();
    seewo_service_freezeui.push("SeewoFreezeUI.exe");
    std::fs::write(seewo_service_freezeui.clone(), seewo_freeze_ui_bin)
        .expect("错误：无法写入文件 SeewoFreezeUI.exe");

    let _ = std::process::Command::new(seewo_service_freezeui)
        .arg("--startup-with-main-window")
        .spawn();
}


#[cfg(not(target_os = "windows"))]
fn main() {
    println!("兄啊，你{}哪来希沃管家的冰点还原啊（恼）", std::env::consts::OS);
}