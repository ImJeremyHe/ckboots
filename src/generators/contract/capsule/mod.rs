pub fn write_capsule(proj_name: &str, contracts: &[&'static str]) {
    let p = env!("CARGO_MANIFEST_DIR");
    let save_path = format!("{proj_name}-contracts");
    let path = std::path::Path::new(p);
    let dir = path.join(save_path);

    let _ = std::fs::remove_dir_all(dir.clone());
    std::fs::create_dir_all(dir.clone()).unwrap();

    let capsule_toml = get_capsule_toml(contracts);
    std::fs::write(dir.join("capsule.toml"), capsule_toml).unwrap();

    let deployment_toml = include_str!("./deployment.toml");
    std::fs::write(dir.join("deployment.toml"), deployment_toml).unwrap();

    let cargo_toml = get_cargo_toml(contracts);
    std::fs::write(dir.join("Cargo.toml"), cargo_toml).unwrap();

    std::fs::create_dir_all(dir.join("contracts")).unwrap();
}

fn get_capsule_toml(contracts: &[&str]) -> String {
    let prelude = r#"version = "0.9.0"
deployment = "deployment.toml"

[[contract]]
name = "_entry"
template_type = "Rust"
"#;
    contracts
        .into_iter()
        .fold(String::from(prelude), |mut prev, c| {
            let contract = format!(
                r#"[[contracts]]
name = "{c}"
template_type = "Rust"
"#
            );
            prev.push_str(&contract);
            prev
        })
}

fn get_cargo_toml(contracts: &[&str]) -> String {
    let members = contracts.into_iter().fold(
        String::from(r#""contracts/types", "contracts/_entry","#),
        |mut prev, c| {
            let member = format!(r#""contracts/{}","#, c);
            prev.push_str(&member);
            prev
        },
    );
    format!(
        r#"
[workspace]
members = [{members}]

[profile.release]
overflow-checks = true
opt-level = 's'
lto = false
codegen-units = 1
panic = 'abort'
"#
    )
}
