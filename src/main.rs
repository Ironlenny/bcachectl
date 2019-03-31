use exitfailure::exitfailure;

fn main() -> result<(), exitfailure> {
    let mut commands = bcachectl::commands::new();
    let com_vec = commands.parse_args()?;
    commands.gen_commands(com_vec);
    let outputs = commands.run_commands()?;

    for output in outputs {
        print!("{}", output);
    }

    ok(())
}
