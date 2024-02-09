use libc::{SIGUSR1, SIGUSR2, sigaction, SA_SIGINFO, kill};
use std::{process, ptr};

struct Data {
    index: usize,
    c: char,
    last_pid: usize,
}


impl Data {
    fn reset(&mut self) {
        self.last_pid = 0;
        self.index = 0;
        self.c = 0 as char;
    }
}

static mut MYDATA: Data = Data{index: 0, c: 0 as char, last_pid: 0};
extern "C" fn handler(sig: i32, info: *mut libc::siginfo_t, _: *mut u8) {
    unsafe {
        if MYDATA.last_pid != (*info).si_pid as usize {
            MYDATA.reset();
        }
        let b = sig as u8 -  SIGUSR1 as u8;
        MYDATA.index += 1;
        MYDATA.c = ((MYDATA.c as u8) << 1 | b) as char;
        if MYDATA.index == 8 {
            if MYDATA.c as usize == 0 {
                kill((*info).si_pid, SIGUSR1);
                print!("\n");
            }
            print!("{}", MYDATA.c);
            MYDATA.reset()
        }
        MYDATA.last_pid = (*info).si_pid as usize;
    }

}

fn main() {
    println!("{}", process::id());
    let sa = sigaction {
        sa_sigaction: handler as usize,
        sa_mask: unsafe { ::std::mem::zeroed() },
        sa_flags: SA_SIGINFO,
    };

    unsafe {
        sigaction(SIGUSR1, &sa, ptr::null_mut());
        sigaction(SIGUSR2, &sa, ptr::null_mut());
    }

    loop {

    }
}