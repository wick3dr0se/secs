use ui_test::{Config, dependencies::DependencyBuilder};

fn main() -> ui_test::Result<()> {
    if cfg!(miri) {
        return Ok(());
    }
    let mut config = Config::rustc("tests/ui");
    let mut deps = DependencyBuilder::default();
    deps.crate_manifest_path = "ui_test_deps/Cargo.toml".into();
    config.comment_defaults.base().add_custom("deps", deps);

    config.path_stderr_filter(&std::env::current_dir().unwrap(), "$DIR");
    ui_test::run_tests(config)
}
