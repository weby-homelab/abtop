//! Binary entry point. All logic lives in the `llm_top` library crate so it can
//! be reused in-process by other tools (see `src/lib.rs` and `src/snapshot.rs`).

fn main() -> std::io::Result<()> {
    llm_top::run()
}
