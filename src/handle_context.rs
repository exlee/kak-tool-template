use std::os::unix::fs::PermissionsExt;
use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};

pub struct Context {
    pub(crate) tools: HashMap<String, Option<PathBuf>>,
}

impl Context {
    pub fn get_tool(&self, key: &str) -> Option<PathBuf> {
        self.tools.get(key).cloned().unwrap_or(None)
    }
    pub fn get_or_create_fifo_path(key: &str) -> std::io::Result<PathBuf> {
        let path = Self::get_fifo_path(key);

        if let Err(e) = std::fs::remove_file(&path) {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(e);
            }
        }

        // Create the new FIFO
        nix::unistd::mkfifo(&path, nix::sys::stat::Mode::from_bits(0o600).unwrap())?;

        Ok(path)
    }
    pub fn get_fifo_path(key: &str) -> PathBuf {
        Path::new("/tmp").join(format!("kak-tool-{}", key))
    }

    pub fn new(required_tools: &[&str]) -> Result<Self, String> {
        let mut tools = HashMap::new();

        for &tool in required_tools {
            tools.insert(tool.to_string(), find_binary(tool));
        }

        Ok(Self { tools })
    }
}

fn find_binary(name: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(name);
            if full_path.is_file() && is_executable(&full_path) {
                Some(full_path)
            } else {
                None
            }
        })
    })
}

fn is_executable(path: &Path) -> bool {
    std::fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}
