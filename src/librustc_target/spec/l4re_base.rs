use crate::spec::{LinkArgs, LinkerFlavor, PanicStrategy, TargetOptions};
//use std::process::Command;

pub fn opts() -> TargetOptions {
    let mut args = LinkArgs::new();
    args.insert(LinkerFlavor::Gcc, vec![]);

    TargetOptions {
        executables: true,
        has_elf_tls: false,
        panic_strategy: PanicStrategy::Abort,
        linker: Some("l4-bender".to_string()),
        pre_link_args: args,
        target_family: Some("unix".to_string()),
        ..Default::default()
    }
}
