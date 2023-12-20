use std::ffi::CString;
use clap::Parser;
use nix::sched::{unshare, CloneFlags};
use nix::unistd::{setuid, Uid, Gid, setgid, execvp};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    userid: Option<u32>,
    #[arg(short, long)]
    groupid: Option<u32>,
    #[arg(last = true, required = true)]
    program: Vec<String>
}

fn main() {
    let args = Args::parse();

    unshare(CloneFlags::CLONE_NEWNET).expect("Failed to unshare net");

    if let Some(v) = args.groupid {
        let group_id = Gid::from(v);
        setgid(group_id).expect(&format!("Failed to set gid to {}", v));
    }
    if let Some(v) = args.userid {
        let user_id = Uid::from(v);
        setuid(user_id).expect(&format!("Failed to set uid to {}", v) );
    }
    
    let program: Vec<CString> = args.program.iter().map(|v| CString::new(v.clone()).unwrap()).collect();

    execvp(&program[0], &program).expect("Failed to exec");
}
