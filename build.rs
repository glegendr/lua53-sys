use std::{env, path::PathBuf};

use bindgen::MacroTypeVariation;

fn main() {
    let lua_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("lua");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let target_os = env::var("CARGO_CFG_TARGET_OS");
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY");

    let mut cc_config = cc::Build::new();
    cc_config.warnings(false);

    if target_os == Ok("linux".to_string()) {
        cc_config.define("LUA_USE_LINUX", None);
    } else if target_os == Ok("macos".to_string()) {
        cc_config.define("LUA_USE_MACOSX", None);
    } else if target_os == Ok("ios".to_string()) {
        cc_config.define("LUA_USE_IOS", None);
    } else if target_family == Ok("unix".to_string()) {
        cc_config.define("LUA_USE_POSIX", None);
    } else if target_family == Ok("windows".to_string()) {
        cc_config.define("LUA_USE_WINDOWS", None);
    }

    let mut cc_config_build = cc_config.include(&lua_dir);

    cc_config_build = cc_config_build
        .file(lua_dir.join("lapi.c"))
        .file(lua_dir.join("lauxlib.c"))
        .file(lua_dir.join("lbaselib.c"))
        .file(lua_dir.join("lcode.c"))
        .file(lua_dir.join("lcorolib.c"))
        .file(lua_dir.join("lctype.c"))
        .file(lua_dir.join("ldebug.c"))
        .file(lua_dir.join("ldo.c"))
        .file(lua_dir.join("ldump.c"))
        .file(lua_dir.join("lfunc.c"))
        .file(lua_dir.join("lgc.c"))
        .file(lua_dir.join("llex.c"))
        .file(lua_dir.join("lmathlib.c"))
        .file(lua_dir.join("lmem.c"))
        .file(lua_dir.join("loadlib.c"))
        .file(lua_dir.join("lobject.c"))
        .file(lua_dir.join("lopcodes.c"))
        .file(lua_dir.join("lparser.c"))
        .file(lua_dir.join("lstate.c"))
        .file(lua_dir.join("lstring.c"))
        .file(lua_dir.join("lstrlib.c"))
        .file(lua_dir.join("ltable.c"))
        .file(lua_dir.join("ltablib.c"))
        .file(lua_dir.join("ltm.c"))
        .file(lua_dir.join("lundump.c"))
        .file(lua_dir.join("lutf8lib.c"))
        .file(lua_dir.join("lvm.c"))
        .file(lua_dir.join("lzio.c"));

    let libc = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("libc");

    if cfg!(feature = "baremetal") {
        cc_config_build
            .file(lua_dir.join("linit_baremetal.c"))
            .file(lua_dir.join("ldblib_baremetal.c"))
            .file(lua_dir.join("loslib_baremetal.c"))
            .file(lua_dir.join("liolib_baremetal.c"))
            .cpp(true)
            .cpp_link_stdlib(None)
            .include(&libc)
            .file(libc.join("libc_utils.cpp"))
            .file(libc.join("libcpp_throw.cpp"))
            .flag("-fno-rtti")
            .flag("-fexceptions")
            .flag("-fwasm-exceptions")
            .flag("-mllvm")
            .flag("-wasm-use-legacy-eh=false");
        cc::Build::new()
            .file(libc.join("snprintf.c"))
            .include(&libc)
            .out_dir(out.join("lib"))
            .compile("snprintf");
    } else {
        cc_config_build
            .file(lua_dir.join("ldblib.c"))
            .file(lua_dir.join("liolib.c"))
            .file(lua_dir.join("linit.c"))
            .file(lua_dir.join("loslib.c"));
    }

    cc_config_build.out_dir(out.join("lib")).compile("lua53");

    let target = env::var("TARGET").unwrap();
    let mut bindings = bindgen::builder()
        .header("lua/lua.h")
        .header("lua/lualib.h")
        .header("lua/lauxlib.h")
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .clang_arg("-fvisibility=default")
        .clang_arg(format!("--target={}", target))
        .blocklist_type("lua_Debug")
        .blocklist_type("lua_CFunction")
        .blocklist_type("lua_Alloc")
        .blocklist_type("lua_Hook")
        .blocklist_type("lua_KFunction")
        .blocklist_type("lua_Writer")
        .blocklist_item("LUA_COLIBNAME")
        .blocklist_item("LUA_TABLIBNAME")
        .blocklist_item("LUA_IOLIBNAME")
        .blocklist_item("LUA_OSLIBNAME")
        .blocklist_item("LUA_STRLIBNAME")
        .blocklist_item("LUA_BITLIBNAME")
        .blocklist_item("LUA_MATHLIBNAME")
        .blocklist_item("LUA_DBLIBNAME")
        .blocklist_item("LUA_LOADLIBNAME")
        .blocklist_item("LUA_LOADED_TABLE")
        .blocklist_item("LUA_PRELOAD_TABLE")
        .blocklist_item("LUA_UTF8LIBNAME")
        .blocklist_function("lua_error")
        .blocklist_function("lua_resume")
        .blocklist_function("lua_sethook")
        .blocklist_item("LUA_RIDX_MAINTHREAD")
        .blocklist_item("LUA_RIDX_GLOBALS")
        .blocklist_item("LUA_RIDX_LAST")
        .override_abi(bindgen::Abi::CUnwind, ".*");

    if cfg!(feature = "baremetal") {
        bindings = bindings.clang_arg(format!("-I{}", libc.display()));
    }
    bindings
        .use_core()
        .generate()
        .unwrap()
        .write_to_file(out.join("bindings.rs"))
        .unwrap();
    println!("cargo::rerun-if-changed=libc/libc_utils.cpp");
    println!("cargo::rerun-if-changed=libc/libcpp_throw.cpp");
}
