use roblox_install::RobloxStudio;

#[cfg(target_os = "windows")]
#[test]
fn test_windows() {
    let studio = RobloxStudio::locate().unwrap();

	assert!(studio
        .root_path()
        .to_string_lossy()
        .contains("Roblox"));

    assert!(studio
        .built_in_plugins_path()
        .to_string_lossy()
        .contains("BuiltInPlugins"));

    assert!(studio
        .exe_path()
        .to_string_lossy()
        .contains("RobloxStudioBeta.exe"));
}