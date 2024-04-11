mod s01_unrecoverable_errors_with_panic;
mod s02_recoverable_errors_with_result;

fn main() {
    println!("Hello, world!");

    println!("\n#9.1# Unrecoverable Errors with panic!");
    // s01_unrecoverable_errors_with_panic::body();
    // s01_unrecoverable_errors_with_panic::using_a_panic_backtrace();
    // s02_recoverable_errors_with_result::body();
    // s02_recoverable_errors_with_result::matching_on_different_errors();
    // s02_recoverable_errors_with_result::alternatives_to_using_match_with_result_t_e();
    s02_recoverable_errors_with_result::shortcuts_for_panic_on_error_unwrap_and_expect();
}  
