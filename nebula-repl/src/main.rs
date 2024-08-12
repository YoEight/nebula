use glyph::Options;
use nebula_core::parse;
use nebula_runtime::derivation;

fn main() -> eyre::Result<()> {
    let options = Options::default()
        .prompt("π>")
        .header(include_str!("header.txt"))
        .author("Yo Eight")
        .version("master");

    let mut inputs = glyph::in_memory_inputs(options)?;

    while let Some(input) = inputs.next_input()? {
        match input {
            glyph::Input::Exit => break,
            glyph::Input::Command(_) => continue,
            glyph::Input::String(code) => match parse(code) {
                Err(e) => println!("{}", e),
                Ok(prg) => match derivation(prg) {
                    Err(e) => println!("{}", e),
                    Ok(v) => println!("{}", v),
                },
            },
        }
    }

    Ok(())
}
