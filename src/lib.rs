pub mod syscalls;

pub mod syscall {
    use std::arch::asm;
    pub fn syscall_3(syscall: u64, arg1: u64, arg2: u64, arg3: u64) -> i64 {
        unsafe {
            let res;
            asm!(
                "syscall",
                in("rax") syscall,
                in("rdi") arg1,
                in("rsi") arg2,
                in("rdx") arg3,
                lateout("rax") res,
            );
            res
        }
    }
    pub fn syscall_6(
        syscall: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> i64 {
        unsafe {
            let res;
            asm!(
                "syscall",
                in("rax") syscall,
                in("rdi") arg1,
                in("rsi") arg2,
                in("rdx") arg3,
                in("r10") arg4,
                in("r8") arg5,
                in("r9") arg6,
                lateout("rax") res,
            );
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syscalls;

    use super::syscall::{syscall_3, syscall_6};
    use std::process::Command;
    fn clean_tests() {
        Command::new("sh")
            .arg("-c")
            .arg("rm -rf test-*")
            .output()
            .unwrap();
    }

    #[test]
    fn open_write_close_syscall_6() {
        let path = "test-6.file".to_string();
        let data = b"you look beautiful today";

        let fd = syscall_6(
            syscalls::SYS_OPEN,
            path.as_ptr() as u64,
            syscalls::O_CREAT | syscalls::O_RDWR,
            syscalls::MODE_PERM,
            0,
            0,
            0,
        );

        println!("open: {}", fd);
        assert!(fd > 0);
        let mut ret = syscall_6(
            syscalls::SYS_WRITE,
            fd as u64,
            data.as_ptr() as u64,
            data.len() as u64,
            0,
            0,
            0,
        );
        assert_eq!(ret, 24);
        println!("write: {}", ret);
        ret = syscall_6(syscalls::SYS_CLOSE, fd as u64, 0 as u64, 0 as u64, 0, 0, 0);
        assert_eq!(ret, 0);
        println!("close: {}", ret);
        clean_tests();
    }
    #[test]
    fn open_write_close_syscall_3() {
        let path = "test-3.file".to_string();
        let data = b"you look beautiful today";

        let fd = syscall_3(
            syscalls::SYS_OPEN,
            path.as_ptr() as u64,
            syscalls::O_CREAT | syscalls::O_RDWR,
            syscalls::MODE_PERM,
        );

        println!("open: {}", fd);
        assert!(fd > 0);
        let mut ret = syscall_3(
            syscalls::SYS_WRITE,
            fd as u64,
            data.as_ptr() as u64,
            data.len() as u64,
        );
        assert_eq!(ret, 24);
        println!("write: {}", ret);
        ret = syscall_3(syscalls::SYS_CLOSE, fd as u64, 0 as u64, 0 as u64);
        assert_eq!(ret, 0);
        println!("close: {}", ret);
        clean_tests();
    }
}
