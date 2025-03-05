use std::path::Path;

use ui_test::{Config, dependencies::DependencyBuilder};

fn main() -> ui_test::Result<()> {
    if cfg!(miri) {
        return Ok(());
    }
    let mut config = Config::rustc("tests/ui");
    let mut deps = DependencyBuilder::default();
    deps.crate_manifest_path = "ui_test_deps/Cargo.toml".into();
    config.comment_defaults.base().add_custom("deps", deps);

    config.path_stderr_filter(&Path::new(file!()).parent().unwrap(), "$DIR");
    config.stderr_filter("(src/.*\\.rs):[0-9]+:[0-9]+", "$1");
    config.stderr_filter("[0-9][0-9][0-9] \\|", "LLL |");
    config.stderr_filter("[0-9][0-9] \\|", "LL |");
    config.stderr_filter("[0-9] \\|", "L |");
    ui_test::run_tests(config)
}
