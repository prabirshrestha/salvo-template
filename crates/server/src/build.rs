use ructe::{Result, Ructe};

fn main() -> Result<()> {
    let mut ructe = Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_files("assets")?;
    statics.add_sass_file("stylesheets/style.scss")?;
    ructe.compile_templates("templates")?;
    Ok(())
}
