#[cfg(not(target_arch = "orgin-main"))]
extern crate napi_build;

fn main() {
  #[cfg(not(target_arch = "orgin-main"))]
  napi_build::setup();
}
