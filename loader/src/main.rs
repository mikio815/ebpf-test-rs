use anyhow::Result;
use libbpf_rs::{Link, ObjectBuilder, ProgramType};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let bpf_path = std::env::var("BPF_OBJECT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target/bpf/lsm_block.bpf.o"));

    let mut obj = ObjectBuilder::default().open_file(&bpf_path)?.load()?;
    let mut links: Vec<Link> = Vec::new();

    for prog in obj.progs_iter_mut() {
        if matches!(prog.prog_type(), ProgramType::Lsm) {
            let name = prog.name().to_string();
            let link = prog.attach_lsm()?;
            links.push(link);
            println!("attached LSM program: {name}");
        }
    }

    println!("running");
    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
