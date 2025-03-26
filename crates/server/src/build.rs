fn main() -> anyhow::Result<()> {
    let gitcl = vergen_gitcl::GitclBuilder::all_git()?;

    if let Err(error) = vergen::Emitter::default()
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

    let mut ructe = ructe::Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    // TODO: find a home for compiled vs source assets such that source doesn't become public.
    statics.add_files("assets/build")?;
    ructe.compile_templates("templates")?;

    Ok(())
}
