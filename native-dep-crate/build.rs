fn main() {
    // This is where the bug fires: cc-rs resolves CC/AR via
    // CC_<build-triple> (first match wins) but resolves CFLAGS by
    // *concatenating* every matching var - generic CFLAGS (HOST-scoped)
    // gets appended after CFLAGS_<build-triple> (BUILD-scoped), even
    // when the latter is set and clean. See cc-1.4.4 src/lib.rs,
    // `envflags()` vs `getenv_with_target_prefixes()`.
    cc::Build::new().file("src/dummy.c").compile("dummy");
}
