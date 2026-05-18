// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            powershell,
            tarefa,
            turnoff,
            restart,
            sleep,
            formatarc,
            formatard,
            formatarf,
            formatarg,
            attdriver,
            open_cmd,
            rodar,
            scandisk,
            scriptmenu,
            limpeza,
            redeip,
            cunitc,
            dunitd,
            funite,
            gunitg,
            dvscode,
            ddiscord,
            dspotify,
            dsteam,
            dzapzap,
            dfirefox,
            ram
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn open_cmd() {
    Command::new("cmd")
        .args(["/C", "start", "", "cmd"])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn rodar() {
    Command::new("cmd")
        .args(["/C", "start", "", "cmd"])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn scandisk() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "chkdsk",
            "/f",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn scriptmenu() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "../../../Sistema-.bat-main/initialcommit.bat",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn limpeza() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "cleanmgr",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn attdriver() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "winget",
            "upgrade",
            "--all",
            "--include-unknown",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn formatarc() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "format",
            "C:",
            "/FS:NTFS",
            "/Q",
            "/V:NovaUnidade",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn formatard() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "format",
            "D:",
            "/FS:NTFS",
            "/Q",
            "/V:NovaUnidade",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn formatarf() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "format",
            "F:",
            "/FS:NTFS",
            "/Q",
            "/V:NovaUnidade",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn formatarg() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "format",
            "G:",
            "/FS:NTFS",
            "/Q",
            "/V:NovaUnidade",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn turnoff() {
    Command::new("cmd")
        .args([
            "/C",
            "shutdown /s /t 5"
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn restart() {
    Command::new("cmd")
        .args([
            "/C",
            "shutdown /r /t 5"
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn sleep() {
    Command::new("cmd")
        .args([
            "/C",
            "rundll32.exe powrprof.dll,SetSuspendState 0,1,0"
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn redeip() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "ipconfig",
            "/release",
            "&&",
            "ipconfig",
            "/renew",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn cunitc() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "chkdsk",
            "C:",
            "/f",
            "/r",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn dunitd() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "chkdsk",
            "D:",
            "/f",
            "/r",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn funite() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "chkdsk",
            "F:",
            "/f",
            "/r",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn gunitg() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "chkdsk",
            "G:",
            "/f",
            "/r",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn dvscode() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "winget",
            "install",
            "--id=Microsoft.VisualStudioCode",
            "-e",
            "--source=winget",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn ddiscord() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "winget",
            "install",
            "--id=Discord.Discord",
            "-e",
            "--source=winget",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn dspotify() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "winget",
            "install",
            "--id=Spotify.Spotify",
            "-e",
            "--source=winget",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn dsteam() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "winget",
            "install",
            "--id=Valve.Steam",
            "-e",
            "--source=winget",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn dzapzap() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
             "",
            "cmd",
            "/K",
            "winget",
            "install",
            "--id=WhatsApp.WhatsAppDesktop",
            "-e",
            "--source=winget",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn dfirefox() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "winget",
            "install",
            "--id=Mozilla.Firefox",
            "-e",
            "--source=winget",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn ram() {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            "cmd",
            "/K",
            "systeminfo",
            "|",
            "findstr",
            "/C:Total Physical Memory",
        ])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn tarefa() {
    Command::new("cmd")
        .args(["/C", "start", "", "taskmgr"])
        .spawn()
        .unwrap();
}

#[tauri::command]
fn powershell() {
    Command::new("cmd")
        .args(["/C", "start", "", "powershell"])
        .spawn()
        .unwrap();
}