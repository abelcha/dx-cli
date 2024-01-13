pub mod my_module {
    use std::ffi::CString;
    use std::path::PathBuf;
    #[link(name = "fffs")]
    extern "C" {
        fn getFFFS(apath: *const libc::c_char) -> libc::c_longlong;
    }

    pub fn get_fffs(path: &PathBuf) -> u64 {
        let path_str = path.as_os_str().to_str().unwrap();

        let c_path = CString::new(path_str).expect("CString::new failed");
        let resp = unsafe { getFFFS(c_path.as_ptr()) };
        return resp.try_into().unwrap();
    }

}
