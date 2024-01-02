fn main() {
  println!("cargo:rustc-link-lib=static=apple_script_bridge");
  println!("cargo:rustc-link-search=native=bridge");
}