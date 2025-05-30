use dirs::home_dir;
use std::fs;
use std::path::Path;

pub fn init_storage() -> anyhow::Result<()> {
    let mut path = home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
    path.push(".notebot");
    fs::create_dir_all(&path)?;
    path.push("tasks.json");
    if !Path::new(&path).exists() {
        fs::write(&path, "[]")?;
    }
    Ok(())
}

pub fn get_tasks_path() -> anyhow::Result<String> {
    let mut path = home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
    path.push(".notebot/tasks.json");
    Ok(path.to_string_lossy().to_string())
}
