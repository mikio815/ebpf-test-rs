use nix::unistd::Pid;
use std::env;
use std::fs;
use object::{Object, ObjectSymbol};

type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

fn get_pid() -> Pid {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: program <pid>");
        std::process::exit(1);
    }

    let pid_str = &args[1];

    let pid: i32 = match pid_str.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("PID must be integer");
            std::process::exit(1);
        }
    };

    Pid::from_raw(pid)
}

fn get_base_addr(pid: Pid, exe_path: &str) -> BoxResult<u64> {
    let map_path = format!("/proc/{}/maps", pid.as_raw());
    let contents = fs::read_to_string(map_path)?;

    for line in contents.lines() {
        if line.contains(exe_path) {
            let base_addr = line
                .split('-')
                .next()
                .ok_or("invalid line")
                .and_then(|s| u64::from_str_radix(s, 16).map_err(|_| "invalid address"))?;
            return Ok(base_addr);
        }
    }
    Err("exe mapping not found".into())
}

fn get_offset(path: &str, symbol: &str) -> BoxResult<u64> {
    let bin = fs::read(path)?;
    let obj_file = object::File::parse(&*bin)?;

    for sym in obj_file.symbols() {
        if sym.name()? == symbol {
            return Ok(sym.address());
        }
    }

    Err("symbol not found".into())
}

fn get_exe_path(pid: Pid) -> BoxResult<String> {
    let link = format!("/proc/{}/exe", pid.as_raw());
    Ok(fs::read_link(link)?.to_string_lossy().into_owned())
}

fn write_memory(pid: Pid, addr: u64, val: i32) -> Result<(), std::io::Error> {
    let val_bytes = val.to_ne_bytes();

    let local_iov = libc::iovec {
        iov_base: val_bytes.as_ptr() as *mut libc::c_void,
        iov_len: std::mem::size_of::<i32>(),
    };

    let remote_iov = libc::iovec {
        iov_base: addr as *mut libc::c_void,
        iov_len: std::mem::size_of::<i32>(),
    };

    let result = unsafe {
        libc::process_vm_writev(
            pid.as_raw(),
            &local_iov,
            1,
            &remote_iov,
            1,
            0,
        )
    };

    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn main() -> BoxResult<()> {
    let pid = get_pid();

    let exe_path = get_exe_path(pid)?;
    let base_addr = get_base_addr(pid, &exe_path)?;
    let offset = get_offset(&exe_path, "g_hp")?;

    let target_addr = base_addr + offset;
    println!("Target Address: {:x}", target_addr);

    let val: i32 = 100;

    loop {
        write_memory(pid, target_addr, val)?;

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
