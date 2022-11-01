mod examples;

fn main() {
    // Create a collection of command line arguments
    let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);
    if args.len() > 1 {
        let command = &args[1];
        examples::pattern_match_switch_statement::run(command);
    } else {
        println!(
            "\
Usage: how_to_rust [command]
  Each command runs an example file named after the command.
  (This CLI uses the `pattern_match_switch_statement` example.)

Commands:
  console_log
  get_home_dir
  import_function
  oop
"
        );
    }
}
