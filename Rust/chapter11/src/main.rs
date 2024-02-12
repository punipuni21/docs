mod anyhow;
mod baric;
mod original_error;
mod panic;
mod this_error;
fn main() {
    // baric::use_parse_01();
    // baric::use_error_kind();
    // original_error::use_parse_02();
    // this_error::use_parse_03();
    // anyhow::use_parse04();
    panic::use_expect();
}
