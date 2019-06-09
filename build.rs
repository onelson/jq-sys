#[cfg(feature = "bundled")]
extern crate jq_src;
#[cfg(feature = "pkg-config")]
extern crate pkg_config;

#[cfg(feature = "bundled")]
fn build_bundled() {
    jq_src::build().expect("autotools build").print_cargo_metadata();
}

#[cfg(not(feature = "bundled"))]
fn build_bundled() {} // noop

mod non_bundled {
    use std::env;

    #[cfg(feature = "pkg-config")]
    pub fn configured_by_pkg_config(lib_cfg: &LibConfig) -> bool {
        let got_jq = pkg_config::probe_library("libjq");
        if !lib_cfg.no_onig {
            let got_onig = pkg_config::probe_library("libonig");
            got_jq.is_ok() && got_onig.is_ok()
        } else {
            got_jq.is_ok()
        }
    }

    #[cfg(not(feature = "pkg-config"))]
    pub fn configured_by_pkg_config(_lib_cfg: &LibConfig) -> bool {
        false
    }

    /// Collection of options for linking to jq/oniguruma.
    ///
    /// If the lib dir is not set, we'll fallback to trying to use pkg-config.
    #[derive(Debug)]
    pub struct LibConfig {
        /// location of the lib dir
        pub jq_lib_dir: Option<String>,
        /// whether or not to link statically
        pub jq_lib_static: bool,
        /// whether or not to link against oniguruma
        pub no_onig: bool,
        /// location of onigurama, ignored if `no_onig` is `true`.
        /// falls back to `jq_lib_dir` when unset.
        pub onig_lib_dir: Option<String>,
        /// falls back to the value of `jq_lib_static` when unset.
        pub onig_lib_static: bool,
    }

    impl LibConfig {
        pub fn from_env() -> Self {
            let jq_lib_dir = env::var("JQ_LIB_DIR").ok();
            let jq_lib_static = env::var("JQ_LIB_STATIC")
                .ok()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false);

            let onig_lib_dir = env::var("ONIG_LIB_DIR").ok();
            let onig_lib_static = env::var("ONIG_LIB_STATIC")
                .ok()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(jq_lib_static);

            LibConfig {
                jq_lib_dir,
                jq_lib_static,
                onig_lib_dir,
                onig_lib_static,
                no_onig: env::var("JQ_NO_ONIG")
                    .ok()
                    .map(|s| !s.trim().is_empty())
                    .unwrap_or(false),
            }
        }

        pub fn print_link_info(&self) {
            // at this point we should be certain we have `JQ_LIB_DIR` at a minimum.
            println!(
                "cargo:rustc-link-search=native={}",
                &self.jq_lib_dir.as_ref().expect("JQ_LIB_DIR")
            );
            let statik = if self.jq_lib_static { "static=" } else { "" };
            println!("cargo:rustc-link-lib={}{}", statik, "jq");

            if !self.no_onig {
                if let Some(onig_dir) = self.onig_lib_dir.as_ref() {
                    println!("cargo:rustc-link-search=native={}", onig_dir);
                }
                let statik = if self.onig_lib_static { "static=" } else { "" };
                println!("cargo:rustc-link-lib={}{}", statik, "onig");
            }
        }
    }

}

fn main() {
    if cfg!(feature = "bundled") {
        return build_bundled();
    } else {
        let lib_cfg = LibConfig::from_env();
        use non_bundled::{configured_by_pkg_config, LibConfig};

        if lib_cfg.jq_lib_dir.is_some() {
            lib_cfg.print_link_info();
        } else if configured_by_pkg_config(&lib_cfg) {
            return; // pkg_config prints all the stuff for us if the probes are successful
        } else {
            panic!(
                "Unable to find libjq. \
                 Try setting `JQ_LIB_DIR` to specify the location of the lib."
            );
        }
    }
}
