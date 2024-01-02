extern crate libc;
// use libc::c_char;
use std::ffi::CString;

extern "C" {
    fn executeAppleScript(script: *const libc::c_char) -> *const libc::c_char;
    // fn executeAppleScript(script_text: *const c_char);
}

// pub fn run_apple_script(script: &str) {
//     let c_script = CString::new(script).unwrap();
//     unsafe {
//         executeAppleScript(c_script.as_ptr());
//     }
// }
pub fn run_apple_script(script: &str) -> String {
    let c_script = CString::new(script).expect("CString::new failed");
    unsafe {
        let result = executeAppleScript(c_script.as_ptr());
        std::ffi::CStr::from_ptr(result)
            .to_string_lossy()
            .into_owned()
    }
}
fn main() {
    // let script = "display alert \"Hello\" with title \"Message from Tim\"";
    let script: String = format!("tell application \"Finder\" to get the name of the startup disk");
    // let script: String = format!(
    //     "tell application \"Finder\" to get size of (POSIX file \"{}\" as alias)",
    //     path_str
    // );
    // println!("==> script ---{}---", script);
    // let c_script = match CString::new(script) {
    //     Ok(cs) => cs,
    //     Err(_) => return 0,
    // };

    // let raw_output = unsafe { rosascript::run_apple_script(&script.as_str()) };
    let result = run_apple_script(&script.as_str());

    // run_apple_script(str);
    // let result = run_apple_script(&script);
    println!("RESULTS {}", result);

    println!("Hello, world!");
}
