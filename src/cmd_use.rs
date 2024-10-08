use crate::get_venv_path;
use crate::utils::is_valid_env;

pub fn exec(name: &str) -> anyhow::Result<()> {
    let path = get_venv_path()?.join(name);

    if !path.exists() {
        anyhow::bail!("No env `{name}` exists.");
    }

    if !is_valid_env(&path) {
        anyhow::bail!("Invalid env `{name}`");
    }

    #[cfg(target_family = "windows")]
    {
        use std::process::Command;
        if let Err(e) = Command::new("cmd.exe")
            .arg("/k")
            //.arg("/c")
            //.arg("start cmd /k")
            .arg(path.join("Scripts/activate.bat"))
            .status()
        {
            anyhow::bail!("Failed to activate env `{name}`:\n{e}");
        }
    }

    #[cfg(target_family = "unix")]
    {
        // eprintln!("NOT support on *nix");
        // eprintln!("use `source` command instead");
        println!("source {}/{}/bin/activate", path.as_ref().display(), name)
    }

    Ok(())
}
