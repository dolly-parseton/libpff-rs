use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=./build.rs");

    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Clone Source
    if !PathBuf::from("./libpff").exists() {
        Command::new("git")
            .arg("clone")
            .arg("https://github.com/libyal/libpff.git")
            .output()
            .expect("failed to run clone libpff during build process.");
    }

    // Clean Source
    Command::new("./synclibs.sh")
        .current_dir("./libpff")
        .arg("distclean")
        .output()
        .expect("failed to run ./synclibs.sh during build process.");
    // Run ./autogen.sh
    Command::new("./autogen.sh")
        .current_dir("./libpff")
        .output()
        .expect("an error occured while running ./autogen.sh during build process");
    // Run configure with the dst variable
    Command::new("./configure")
        .arg("--enable-shared=no")
        .arg("--enable-static-executables=no")
        .arg(format!("--prefix={}", dst.display()))
        .current_dir("./libpff")
        .output()
        .expect("an error occured while running ./configure during build process");
    // Run make to install the lib to the dst directory
    Command::new("make")
        .arg("install")
        .current_dir("./libpff")
        .output()
        .expect("an error occured while running 'make install' during build process");
    // Configure rustc
    println!("cargo:root={}", dst.display());
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=pff");
    // Generate bindings
    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        // .enable_cxx_namespaces()
        .clang_arg("-I")
        .clang_arg("./libpff/include")
        .clang_arg("-x")
        .clang_arg("c++")
        .header("./libpff/include/libpff.h")
        .rustified_enum("LIBPFF_ACCESS_FLAGS")
        .rustified_enum("LIBPFF_CODEPAGES")
        .rustified_enum("LIBPFF_RECOVERY_FLAGS")
        .rustified_enum("LIBPFF_FILE_TYPES")
        .rustified_enum("LIBPFF_FILE_CONTENT_TYPES")
        .rustified_enum("LIBPFF_ENCRYPTION_TYPES")
        .rustified_enum("LIBPFF_ITEM_TYPES")
        .rustified_enum("LIBPFF_ATTACHMENT_TYPES")
        .rustified_enum("LIBPFF_UNALLOCATED_BLOCK_TYPES")
        .rustified_enum("LIBPFF_NAME_TO_ID_MAP_ENTRY_TYPES")
        .rustified_enum("LIBPFF_ENTRY_VALUE_FLAGS")
        .rustified_enum("LIBPFF_ERROR_DOMAINS")
        .rustified_enum("LIBPFF_ARGUMENT_ERROR")
        .rustified_enum("LIBPFF_CONVERSION_ERROR")
        .rustified_enum("LIBPFF_COMPRESSION_ERROR")
        .rustified_enum("LIBPFF_IO_ERROR")
        .rustified_enum("LIBPFF_INPUT_ERROR")
        .rustified_enum("LIBPFF_MEMORY_ERROR")
        .rustified_enum("LIBPFF_OUTPUT_ERROR")
        .rustified_enum("LIBPFF_RUNTIME_ERROR")
        .rustified_enum("LIBPFF_ATTACHMENT_METHODS")
        .rustified_enum("LIBPFF_MESSAGE_FLAGS")
        .rustified_enum("LIBPFF_MESSAGE_IMPORTANCE_TYPES")
        .rustified_enum("LIBPFF_MESSAGE_PRIORITY_TYPES")
        .rustified_enum("LIBPFF_MESSAGE_SENSITIVITY_TYPES")
        .rustified_enum("LIBPFF_MESSAGE_STATUS_FLAGS")
        .rustified_enum("LIBPFF_MESSAGE_STORE_VALID_FOLDER_MASKS")
        .rustified_enum("LIBPFF_RECIPIENT_TYPES")
        .rustified_enum("LIBPFF_VALUE_TYPES")
        .rustified_enum("LIBPFF_ENTRY_TYPES")
        .generate()
        .expect("an error occurred while generating bindings");

    // Write the bindings to dst
    bindings
        .write_to_file(dst.join("bindings.rs"))
        .expect("an error occurred while writing bindings");
}
