use std::{env, process};
use libc::{c_char, kill, pid_t, sighandler_t, signal, SIGUSR1, SIGUSR2, size_t, usleep};

extern "C" fn handler(_: size_t) {
    println!("message sent successfully");
}

extern "C" fn send_char(pid: &pid_t, c: c_char) {
    for i in (0i8..8i8).rev() {
        println!("{i}");
        let bin: i8 = c as i8 >> i & 1;
        println!("{bin}");
        match bin {
            1 => unsafe {kill(*pid, SIGUSR2);},
            0 => unsafe {kill(*pid, SIGUSR1);},
            _ => panic!("bin error")
        }
       unsafe { usleep(500); }
    }
}

extern "C" fn send_message(pid: &pid_t, message: &str) {
    for c in message.chars() {
        send_char(pid, c as c_char);
    }
    send_char(pid, 0 as c_char);
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!("{}", process::id());

    ///////////////////////////////////////////////
    // TODO: remove in prod
    // println!("{:?}", args);
    // if args.len() as i32 != 2i32 {
    //     panic!("missing arguments")
    // }
    args.push("20810".to_owned());
    args.push("anas jaidi".to_owned());
    //////////////////////////////////////////////
    unsafe {signal(SIGUSR1, handler as sighandler_t);}

    let pid: pid_t = args[0].parse().unwrap();
    let message = args[1].clone();
    send_message(&pid, &message);
}