pub fn write_types(save_path: &str, types: Vec<&str>) {
    let mut content = include_str!("../../on_chain.rs").to_string();
    types.into_iter().for_each(|s| {
        content.push_str(s);
    });

    let p = env!("CARGO_MANIFEST_DIR");
    let target = std::path::Path::new(p);
    let types_dir = target.join(save_path).join("types");
    let _ = std::fs::remove_dir_all(types_dir.clone());
    std::fs::create_dir_all(types_dir.clone()).unwrap();

    std::fs::write(
        types_dir.join("Cargo.toml"),
        r#"
[package]
name = "contracts"
version = "0.1.0"
edition = "2021"
"#
        .trim_start(),
    )
    .unwrap();

    let src_dir = types_dir.join("src");
    std::fs::create_dir_all(src_dir.clone()).unwrap();
    std::fs::write(src_dir.join("lib.rs"), content).unwrap();
}
