use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    let mut commands = bcachectl::Commands::new();
    let com_vec = commands.parse_args()?;
    commands.gen_commands(com_vec);
    let outputs = commands.run_commands()?;

    for output in outputs {
        print!("{}", output);
    }

    Ok(())
}
