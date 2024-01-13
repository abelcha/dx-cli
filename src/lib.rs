pub mod my_module {
    use once_cell::sync::Lazy;
    use std::ffi::CString;
    use std::path::PathBuf;
    use std::sync::Mutex;
    #[link(name = "fffs")]
    extern "C" {
        fn executeAppleScript(script: *const libc::c_char) -> *const libc::c_char;
        fn getFinderItemSize(apath: *const libc::c_char) -> libc::c_longlong;
        fn getFFFS(apath: *const libc::c_char) -> libc::c_longlong;
    }
    static APPLE_SCRIPT_EXECUTOR: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    pub fn run_apple_script(script: &str) -> String {
        let _lock = APPLE_SCRIPT_EXECUTOR.lock().unwrap();
        let c_script = CString::new(script).expect("CString::new failed");
        unsafe {
            let result = executeAppleScript(c_script.as_ptr());
            std::ffi::CStr::from_ptr(result)
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn get_finder_item_size(path: &PathBuf) -> u64 {
        let path_str = path.as_os_str().to_str().unwrap();

        let c_path = CString::new(path_str).expect("CString::new failed");
        let resp = unsafe { getFinderItemSize(c_path.as_ptr()) };
        return resp.try_into().unwrap();
    }

    pub fn get_fffs(path: &PathBuf) -> u64 {
        let path_str = path.as_os_str().to_str().unwrap();

        let c_path = CString::new(path_str).expect("CString::new failed");
        let resp = unsafe { getFFFS(c_path.as_ptr()) };
        return resp.try_into().unwrap();
    }


    pub fn get_folder_size(path: &PathBuf) -> u64 {
        let script_template: &'static str = include_str!("./get-folder-size.template");
        let script = script_template.replace('%', &path.to_string_lossy());
        let result = run_apple_script(&script.as_str());
        result.parse::<u64>().unwrap_or(0)
    }
}
