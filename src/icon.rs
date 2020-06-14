use crate::error::LauncherError;
use cached::proc_macro::cached;
use ini::Ini;
use std::env;
use std::path::PathBuf;

/// attempt to resolve a single icon theme
#[cached]
fn lookup_theme() -> Result<String, LauncherError> {
    let home_directory = env::var("HOME").map_err(|_| LauncherError::ResolveIconThemeError)?;
    let path: PathBuf = [&home_directory, ".config/gtk-3.0/settings.ini"]
        .iter()
        .collect();
    let gtk_settings =
        Ini::load_from_file(path).map_err(|_| LauncherError::ResolveIconThemeError)?;

    Ok(gtk_settings
        .section(Some("Settings"))
        .ok_or(LauncherError::ResolveIconThemeError)?
        .get("gtk-icon-theme-name")
        .ok_or(LauncherError::ResolveIconThemeError)?
        .to_string())
}

#[cached]
pub fn lookup_icon(name: String) -> Result<String, LauncherError> {
    let theme = lookup_theme()?;

    Ok(
        linicon::lookup_icon(theme, name.to_ascii_lowercase(), 64, 1)
            .map_err(|_| LauncherError::ResolveIconThemeError)?
            .next()
            .ok_or(LauncherError::ResolveIconThemeError)?
            .map_err(|_| LauncherError::ResolveIconThemeError)?
            .path
            .to_string_lossy()
            .to_string(),
    )
}
