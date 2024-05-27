use ructe::{Result, Ructe};

fn main() -> Result<()> {
    let mut ructe = Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_files("assets")?;
    statics.add_sass_file("assets/stylesheets/style.scss")?;
    ructe.compile_templates("templates")?;
    Ok(())
}
