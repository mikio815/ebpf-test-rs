use anyhow::Result;
use libbpf_rs::{ObjectBuilder, ProgramType};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let bpf_path = std::env::var("BPF_OBJECT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target/bpf/lsm_block.bpf.o"));

    let mut obj = ObjectBuilder::default().open_file(&bpf_path)?.load()?;

    for prog in obj.progs_iter_mut() {
        if prog.prog_type() == ProgramType::Lsm {
            let name = prog.name().to_string();
            let mut lsm = prog.lsm_mut().expect("lsm attach");
            lsm.attach()?;
            println!("attached LSM program: {name}");
        }
    }

    println!("running");
    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
