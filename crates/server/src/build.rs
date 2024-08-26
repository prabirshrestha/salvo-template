use ructe::Ructe;
use vergen::Emitter;
use vergen_gitcl::GitclBuilder;

fn main() -> anyhow::Result<()> {
    let gitcl = GitclBuilder::all_git()?;

    if let Err(error) = Emitter::default()
        .add_instructions(&gitcl)?
        .fail_on_error()
        .emit()
    {
        println!(
            "cargo:warning=Could not generate git version information: {:?}",
            error
        );
        println!("cargo:rustc-env=VERGEN_GIT_SHA=nogit");
    }

    let mut ructe = Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_files("assets")?;
    statics.add_sass_file("assets/stylesheets/style.scss")?;
    ructe.compile_templates("templates")?;

    Ok(())
}
