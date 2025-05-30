use dirs::config_dir;
use std::fs;
use std::path::Path;

pub fn init_storage() -> anyhow::Result<()> {
    let mut path = config_dir().ok_or_else(|| anyhow::anyhow!("Cannot find config directory"))?;
    path.push("notebot");
    fs::create_dir_all(&path)?;
    path.push("tasks.json");
    // Always write [] to ensure a clean state
    fs::write(&path, "[]")?;
    Ok(())
}

pub fn get_tasks_path() -> anyhow::Result<String> {
    let mut path = config_dir().ok_or_else(|| anyhow::anyhow!("Cannot find config directory"))?;
    path.push("notebot");
    path.push("tasks.json");
    Ok(path.to_string_lossy().to_string())
}
