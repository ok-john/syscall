## syscall

Cargo library for making raw linux system calls.

## API

```rust
pub fn syscall_3(syscall: u64, arg1: u64, arg2: u64, arg3: u64) -> i64
pub fn syscall_6(syscall: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64, arg6: u64) -> i64 
```
## Example

Opening a file, writing to it, closing the file.

```rust
let path = "test-3.file".to_string();
let data = b"you look beautiful today";

let fd = syscall_3(
        syscalls::SYS_OPEN,
        path.as_ptr() as u64,
        syscalls::O_CREAT | syscalls::O_RDWR,
        syscalls::MODE_PERM,
    );

syscall_3(
    syscalls::SYS_WRITE,
    fd as u64,
    data.as_ptr() as u64,
    data.len() as u64,
);

syscall_3(
    syscalls::SYS_CLOSE, 
    fd as u64, 
    0 as u64, 
    0 as u64
);
```
