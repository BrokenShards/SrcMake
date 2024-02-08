// build.rs //

#[cfg(target_os = "windows")]
fn main()
{
	println!("cargo:rustc-link-lib=./resources/res");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
