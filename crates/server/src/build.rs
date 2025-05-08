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

    compile_tailwindcss()?;

    let mut ructe = ructe::Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_file("assets/stylesheets/app.generated.css")?;
    ructe.compile_templates("templates")?;

    Ok(())
}

fn compile_tailwindcss() -> anyhow::Result<()> {
    use std::process::Command;

    #[allow(unused_mut)]
    let mut args = vec![
        "-i",
        "assets/stylesheets/app.css",
        "-o",
        "assets/stylesheets/app.generated.css",
    ];

    // Only minify in release builds (when debug_assertions is not enabled)
    #[cfg(not(debug_assertions))]
    args.push("--minify");

    let status = Command::new("tailwindcss").args(args).status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to run TailwindCSS compiler"));
    }

    Ok(())
}
