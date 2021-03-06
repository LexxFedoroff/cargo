use std::os;
use docopt;

use cargo::ops;
use cargo::core::MultiShell;
use cargo::util::{CliResult, CliError};
use cargo::util::important_paths::find_root_manifest_for_cwd;

docopt!(Options, "
Update dependencies as recorded in the local lock file.

Usage:
    cargo update [options]
    cargo update [options] <spec>

Options:
    -h, --help               Print this message
    -p SPEC, --package SPEC  Package to run benchmarks for
    --aggressive             Force updating all dependencies of <name> as well
    --precise PRECISE        Update a single dependency to exactly PRECISE
    --manifest-path PATH     Path to the manifest to compile
    -v, --verbose            Use verbose output

This command requires that a `Cargo.lock` already exists as generated by
`cargo build` or related commands.

If SPEC is given, then a conservative update of the lockfile will be
performed. This means that only the dependency specified by SPEC will be
updated. Its transitive dependencies will be updated only if SPEC cannot be
updated without updating dependencies.  All other dependencies will remain
locked at their currently recorded versions.

If PRECISE is specified, then --aggressive must not also be specified. The
argument PRECISE is a string representing a precise revision that the package
being updated should be updated to. For example, if the package comes from a git
repository, then PRECISE would be the exact revision that the repository should
be updated to.

If SPEC is not given, then all dependencies will be re-resolved and
updated.

For more information about package id specifications, see `cargo help pkgid`.
",  flag_manifest_path: Option<String>, arg_spec: Option<String>,
    flag_precise: Option<String>, flag_package: Option<String>)

pub fn execute(options: Options, shell: &mut MultiShell) -> CliResult<Option<()>> {
    debug!("executing; cmd=cargo-update; args={}", os::args());
    shell.set_verbose(options.flag_verbose);
    let root = try!(find_root_manifest_for_cwd(options.flag_manifest_path));

    let spec = if options.arg_spec.is_some() {
        let _ = shell.warn("`cargo update foo` has been deprecated in favor \
                            of `cargo update -p foo`. This functionality \
                            will be removed in the future");
        options.arg_spec.as_ref()
    } else {
        options.flag_package.as_ref()
    };

    let mut update_opts = ops::UpdateOptions {
        aggressive: options.flag_aggressive,
        precise: options.flag_precise.as_ref().map(|s| s.as_slice()),
        to_update: spec.map(|s| s.as_slice()),
        shell: shell,
    };

    ops::update_lockfile(&root, &mut update_opts)
        .map(|_| None).map_err(|err| CliError::from_boxed(err, 101))
}

