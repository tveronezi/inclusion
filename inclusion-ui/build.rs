use std::process::Command;

fn main() {
    println!("build ui...");
    for f in [
        "public",
        "src",
        "craco.config.js",
        "package.json",
        "package-lock.json",
        "tailwind.config.js",
    ] {
        println!("cargo:rerun-if-changed=inclusion-ui/{}", f);
    }
    let ui_dir = std::fs::canonicalize("./inclusion-ui").unwrap();
    // npm install
    println!(
        "{:?}",
        Command::new("npm")
            .current_dir(&ui_dir)
            .args(["install"])
            .output()
            .unwrap()
    );
    // npm run build
    println!(
        "{:?}",
        Command::new("npm")
            .current_dir(&ui_dir)
            .args(["run", "build"])
            .output()
            .unwrap()
    );
}
