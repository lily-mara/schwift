use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let dir = env::current_dir().unwrap();
    let hooks = dir.join(Path::new("hooks"));
    let hooks_str = hooks.to_str().unwrap();

    let cmd = &format!("rm -rf .git/hooks && ln -s {} .git/hooks", hooks_str);

    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
}
