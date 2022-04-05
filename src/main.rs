mod print;

fn main() {
    print::run();
    print::run_loop();
    println!("pi value is {}", print::get_pi());
    print::tuple_run();
}
