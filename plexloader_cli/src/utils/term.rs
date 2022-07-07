use console::{style, Style};

pub fn success() -> Style{
    Style::new().green()
}

pub fn info() -> Style {
    Style::new().yellow()
}

pub fn error() -> Style {
    Style::new().red()
}

pub fn print_err(r: anyhow::Result<()>) {
    if let Err(e) = r {
        eprintln!("{}: {}", error().apply_to("Error"), style(&e).bold());
        eprintln!("");
        eprintln!("{}:", info().apply_to("Caused by"));
        e.chain()
            .skip(1)
            .for_each(|cause| eprintln!("\t{}", style(&cause).bold()));
        std::process::exit(1);
    }
}
