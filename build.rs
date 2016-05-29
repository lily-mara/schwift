use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to get cargo manifest directory from env");
    let manifest_path = Path::new(&manifest_dir);

    let real_hooks_path = manifest_path.join(Path::new("hooks"));
    let git_hooks_path = manifest_path.join(".git").join("hooks");

    let real_dir = real_hooks_path.to_str().unwrap();
    let git_dir = git_hooks_path.to_str().unwrap();

    let cmd = &format!("rm -rf {git} && ln -s {real} {git}", git=git_dir, real=real_dir);

    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .unwrap_or_else(|e| panic!("Failed to create symbolic links: {:?}", e));
}
