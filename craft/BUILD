
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test", "rust_doc_test")

rust_library(
    name = "craft",
    srcs = glob([
        "src/**/*.rs",
    ]),
    aliases = aliases(),
    deps = all_crate_deps(),
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    visibility = ["//visibility:public"],
)
