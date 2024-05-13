extern crate cc;

#[cfg(feature = "bindgen")]
extern crate bindgen;

#[cfg(feature = "bindgen")]
#[path = "bindgen.rs"]
mod generate;

use std::env;
use std::path::PathBuf;

const MDB_IDL_LOGN_DEFAULT: u8 = 16;
const MDB_IDL_LOGN: Option<u8> = {
    cfg_if::cfg_if! {
        if #[cfg(feature = "mdb_idl_logn_16")] {
            Some(16)
        } else if #[cfg(feature = "mdb_idl_logn_15")] {
            Some(15)
        } else if #[cfg(feature = "mdb_idl_logn_14")] {
            Some(14)
        } else if #[cfg(feature = "mdb_idl_logn_13")] {
            Some(13)
        } else if #[cfg(feature = "mdb_idl_logn_12")] {
            Some(12)
        } else if #[cfg(feature = "mdb_idl_logn_11")] {
            Some(11)
        } else if #[cfg(feature = "mdb_idl_logn_10")] {
            Some(10)
        } else if #[cfg(feature = "mdb_idl_logn_9")] {
            Some(9)
        } else {
            None
        }
    }
};

macro_rules! warn {
    ($message:expr) => {
        println!("cargo:warning={}", $message);
    };
}

fn main() {
    #[cfg(feature = "bindgen")]
    generate::generate();

    let mut lmdb = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    lmdb.push("lmdb");
    lmdb.push("libraries");
    lmdb.push("liblmdb");

    if cfg!(feature = "with-fuzzer") && cfg!(feature = "with-fuzzer-no-link") {
        warn!("Features `with-fuzzer` and `with-fuzzer-no-link` are mutually exclusive.");
        warn!("Building with `-fsanitize=fuzzer`.");
    }

    let mut builder = cc::Build::new();

    let mdb_idl_logn = MDB_IDL_LOGN.unwrap_or_else(|| {
        warn!("MDB_IDL_LOGN feature not enabled, defaulting to {MDB_IDL_LOGN_DEFAULT}");
        MDB_IDL_LOGN_DEFAULT
    });

    builder
        .define("MDB_IDL_LOGN", Some(mdb_idl_logn.to_string().as_str()))
        .file(lmdb.join("mdb.c"))
        .file(lmdb.join("midl.c"))
        // https://github.com/mozilla/lmdb/blob/b7df2cac50fb41e8bd16aab4cc5fd167be9e032a/libraries/liblmdb/Makefile#L23
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wbad-function-cast")
        .flag_if_supported("-Wuninitialized");

    if cfg!(feature = "posix-sem") {
        builder.define("MDB_USE_POSIX_SEM", None);
    }

    if cfg!(feature = "asan") {
        builder.flag("-fsanitize=address");
    }

    if cfg!(feature = "fuzzer") {
        builder.flag("-fsanitize=fuzzer");
    } else if cfg!(feature = "fuzzer-no-link") {
        builder.flag("-fsanitize=fuzzer-no-link");
    }

    builder.compile("liblmdb.a")
}
