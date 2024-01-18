pub mod fffs {
    use std::ffi::CString;
    use std::path::PathBuf;
    #[link(name = "fffs")]
    extern "C" {
        fn getFinderFastFolderSize(apath: *const libc::c_char) -> libc::c_longlong;
    }

    pub fn get_finder_fast_folder_size(path: &PathBuf) -> u64 {
        let path_str = path.as_os_str().to_str().unwrap();

        let c_path = CString::new(path_str).expect("CString::new failed");
        let resp = unsafe { getFinderFastFolderSize(c_path.as_ptr()) };
        return resp.try_into().unwrap();
    }

}
