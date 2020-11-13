use std::env;
use std::path::PathBuf;
use std::process::Command;

const BUILD_ERROR_MSG: &str = "Unable to build botan.";
const SRC_DIR_ERROR_MSG: &str = "Unable to find the source directory.";
const SRC_DIR: &str = "botan";
const INCLUDE_DIR: &str = "build/include/botan";

macro_rules! pathbuf_to_string {
    ($s: ident) => {
        $s.to_str().expect(BUILD_ERROR_MSG).to_string()
    };
}

macro_rules! add_env_arg {
    ($cnf: ident, $env_name: expr, $arg_name: expr, $is_flag: expr) => {
        if let Ok(val) = env::var($env_name) {
            let arg = if $is_flag {
                format!("{}", $arg_name)
            } else {
                format!("{}={}", $arg_name, val)
            };
            $cnf.arg(&arg);
        }
    };
}

fn configure(build_dir: &str) {
    let mut configure = Command::new("python");
    configure.arg("configure.py");
    configure.arg(format!("--with-build-dir={}", build_dir));
    configure.arg("--build-targets=static");
    configure.arg("--without-documentation");
    configure.arg("--no-install-python-module");
    configure.arg("--distribution-info=https://crates.io/crates/botan-src");
    #[cfg(debug_assertions)]
    configure.arg("--with-debug-info");
    add_env_arg!(configure, "BOTAN_CONFIGURE_OS", "--os", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_CPU", "--cpu", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_CC", "--cc", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_CC_MIN_VERSION", "--cc-min-version", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_CC_BIN", "--cc-bin", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_CC_API_FLAGS", "--cc-abi-flags", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_CXXFLAGS", "--cxxflags", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_EXTRA_CXXFLAGS", "--extra-cxxflags", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_LDFLAGS", "--ldflags", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_AR_COMMAND", "--ar-command", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_AR_OPTIONS", "--ar-options", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_MSVC_RUNTIME", "--msvc-runtime", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_WITH_ENDIAN", "--with-endian", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_WITH_OS_FEATURES", "--with-os-features", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_WITHOUT_OS_FEATURES", "--without-os-features", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_OPTIMIZE_FOR_SIZE", "--optimize-for-size", true);
    add_env_arg!(configure, "BOTAN_CONFIGURE_NO_OPTIMIZATIONS", "--no-optimizations", true);
    add_env_arg!(configure, "BOTAN_CONFIGURE_AMALGAMATION", "--amalgamation", true);
    add_env_arg!(configure, "BOTAN_CONFIGURE_SYSTEM_CERT_BUNDLE", "--system-cert-bundle", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_WITH_LOCAL_CONFIG", "--with-local-config", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_BOOST_LIBRARY_NAME", "--boost-library-name", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_MODULE_POLICY", "--module-policy", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_ENABLE_MODULES", "--enable-modules", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_DISABLE_MODULES", "--disable-modules", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_MINIMIZED_BUILD", "--minimized-build", true);
    add_env_arg!(configure, "BOTAN_CONFIGURE_WITH_OPENSSL", "--with-openssl", true);
    add_env_arg!(configure, "BOTAN_CONFIGURE_WITH_COMMONCRYPTO", "--with-commoncrypto", true);
    add_env_arg!(configure, "BOTAN_CONFIGURE_WITH_SQLITE3", "--with-sqlite3", true);
    add_env_arg!(configure, "BOTAN_CONFIGURE_PROGRAM_SUFFIX", "--program-suffix", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_LIBRARY_SUFFIX", "--library-suffix", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_PREFIX", "--prefix", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_BINDIR", "--bindir", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_LIBDIR", "--libdir", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_MANDIR", "--mandir", false);
    add_env_arg!(configure, "BOTAN_CONFIGURE_INCLUDEDIR", "--includedir", false);
    let status = configure
        .spawn()
        .expect(BUILD_ERROR_MSG)
        .wait()
        .expect(BUILD_ERROR_MSG);
    if !status.success() {
        panic!("configure terminated unsuccessfully");
    }
}

fn make(build_dir: &str) {
    let mut cmd = Command::new("make");
    // Set MAKEFLAGS to the content of CARGO_MAKEFLAGS
    // to give jobserver (parallel builds) support to the
    // spawned sub-make.
    if let Ok(val) = env::var("CARGO_MAKEFLAGS") {
        cmd.env("MAKEFLAGS", val);
    } else {
        eprintln!("Can't set MAKEFLAGS as CARGO_MAKEFLAGS couldn't be read");
    }
    let status = cmd.arg("-f")
        .arg(format!("{}/Makefile", build_dir))
        .arg("libs")
        .spawn()
        .expect(BUILD_ERROR_MSG)
        .wait()
        .expect(BUILD_ERROR_MSG);
    if !status.success() {
        panic!("make terminated unsuccessfully");
    }
}

pub fn build() -> (String, String) {
    let src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(SRC_DIR);
    let build_dir = env::var_os("OUT_DIR").map_or(src_dir.to_owned(), PathBuf::from);
    let build_dir = build_dir.join("botan");
    let include_dir = build_dir.join(INCLUDE_DIR);
    let build_dir = pathbuf_to_string!(build_dir);
    env::set_current_dir(&src_dir).expect(SRC_DIR_ERROR_MSG);
    configure(&build_dir);
    make(&build_dir);
    (build_dir, pathbuf_to_string!(include_dir))
}
