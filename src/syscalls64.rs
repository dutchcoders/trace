pub struct SyscallParameters {
    pub name: String,
    pub sysname: String,
}


#[derive(Debug)]
pub struct Syscall {
    pub number: u16,
    pub name: &'static str,
    pub sysname: &'static str,
    pub file: &'static str, // parameters: Option<Vec<SyscallParameters>>,
}

pub const syscalls64: [Syscall; 314] = [
    Syscall{number: 216, name: "remap_file_pages", file: "sys_remap_file_pages", sysname: "mm/fremap.c"},
    Syscall{number: 217, name: "getdents64", file: "sys_getdents64", sysname: "fs/readdir.c"},
    Syscall{number: 214, name: "epoll_ctl_old", file: "None", sysname: "None"},
    Syscall{number: 215, name: "epoll_wait_old", file: "None", sysname: "None"},
    Syscall{number: 212, name: "lookup_dcookie", file: "sys_lookup_dcookie", sysname: "fs/dcookies.c"},
    Syscall{number: 213, name: "epoll_create", file: "sys_epoll_create", sysname: "fs/eventpoll.c"},
    Syscall{number: 210, name: "io_cancel", file: "sys_io_cancel", sysname: "fs/aio.c"},
    Syscall{number: 211, name: "get_thread_area", file: "None", sysname: "arch/x86/kernel/tls.c"},
    Syscall{number: 165, name: "mount", file: "sys_mount", sysname: "fs/namespace.c"},
    Syscall{number: 264, name: "renameat", file: "sys_renameat", sysname: "fs/namei.c"},
    Syscall{number: 265, name: "linkat", file: "sys_linkat", sysname: "fs/namei.c"},
    Syscall{number: 218, name: "set_tid_address", file: "sys_set_tid_address", sysname: "kernel/fork.c"},
    Syscall{number: 219, name: "restart_syscall", file: "sys_restart_syscall", sysname: "kernel/signal.c"},
    Syscall{number: 133, name: "mknod", file: "sys_mknod", sysname: "fs/namei.c"},
    Syscall{number: 132, name: "utime", file: "sys_utime", sysname: "fs/utimes.c"},
    Syscall{number: 131, name: "sigaltstack", file: "sys_sigaltstack", sysname: "kernel/signal.c"},
    Syscall{number: 130, name: "rt_sigsuspend", file: "sys_rt_sigsuspend", sysname: "kernel/signal.c"},
    Syscall{number: 137, name: "statfs", file: "sys_statfs", sysname: "fs/statfs.c"},
    Syscall{number: 136, name: "ustat", file: "sys_ustat", sysname: "fs/statfs.c"},
    Syscall{number: 135, name: "personality", file: "sys_personality", sysname: "kernel/exec_domain.c"},
    Syscall{number: 134, name: "uselib", file: "None", sysname: "fs/exec.c"},
    Syscall{number: 139, name: "sysfs", file: "sys_sysfs", sysname: "fs/filesystems.c"},
    Syscall{number: 138, name: "fstatfs", file: "sys_fstatfs", sysname: "fs/statfs.c"},
    Syscall{number: 166, name: "umount2", file: "sys_umount", sysname: "fs/namespace.c"},
    Syscall{number: 24, name: "sched_yield", file: "sys_sched_yield", sysname: "kernel/sched/core.c"},
    Syscall{number: 25, name: "mremap", file: "sys_mremap", sysname: "mm/mmap.c"},
    Syscall{number: 26, name: "msync", file: "sys_msync", sysname: "mm/msync.c"},
    Syscall{number: 27, name: "mincore", file: "sys_mincore", sysname: "mm/mincore.c"},
    Syscall{number: 20, name: "writev", file: "sys_writev", sysname: "fs/read_write.c"},
    Syscall{number: 21, name: "access", file: "sys_access", sysname: "fs/open.c"},
    Syscall{number: 22, name: "pipe", file: "sys_pipe", sysname: "fs/pipe.c"},
    Syscall{number: 23, name: "select", file: "sys_select", sysname: "fs/select.c"},
    Syscall{number: 160, name: "setrlimit", file: "sys_setrlimit", sysname: "kernel/sys.c"},
    Syscall{number: 28, name: "madvise", file: "sys_madvise", sysname: "mm/madvise.c"},
    Syscall{number: 29, name: "shmget", file: "sys_shmget", sysname: "ipc/shm.c"},
    Syscall{number: 161, name: "chroot", file: "sys_chroot", sysname: "fs/open.c"},
    Syscall{number: 289, name: "signalfd4", file: "sys_signalfd4", sysname: "fs/signalfd.c"},
    Syscall{number: 0, name: "read", file: "sys_read", sysname: "fs/read_write.c"},
    Syscall{number: 4, name: "stat", file: "sys_newstat", sysname: "fs/stat.c"},
    Syscall{number: 281, name: "epoll_pwait", file: "sys_epoll_pwait", sysname: "fs/eventpoll.c"},
    Syscall{number: 8, name: "lseek", file: "sys_lseek", sysname: "fs/read_write.c"},
    Syscall{number: 283, name: "timerfd_create", file: "sys_timerfd_create", sysname: "fs/timerfd.c"},
    Syscall{number: 163, name: "acct", file: "sys_acct", sysname: "kernel/acct.c"},
    Syscall{number: 285, name: "fallocate", file: "sys_fallocate", sysname: "fs/open.c"},
    Syscall{number: 284, name: "eventfd", file: "sys_eventfd", sysname: "fs/eventfd.c"},
    Syscall{number: 287, name: "timerfd_gettime", file: "sys_timerfd_gettime", sysname: "fs/timerfd.c"},
    Syscall{number: 286, name: "timerfd_settime", file: "sys_timerfd_settime", sysname: "fs/timerfd.c"},
    Syscall{number: 119, name: "setresgid", file: "sys_setresgid", sysname: "kernel/sys.c"},
    Syscall{number: 271, name: "ppoll", file: "sys_ppoll", sysname: "fs/select.c"},
    Syscall{number: 258, name: "mkdirat", file: "sys_mkdirat", sysname: "fs/namei.c"},
    Syscall{number: 120, name: "getresgid", file: "sys_getresgid", sysname: "kernel/sys.c"},
    Syscall{number: 121, name: "getpgid", file: "sys_getpgid", sysname: "kernel/sys.c"},
    Syscall{number: 122, name: "setfsuid", file: "sys_setfsuid", sysname: "kernel/sys.c"},
    Syscall{number: 123, name: "setfsgid", file: "sys_setfsgid", sysname: "kernel/sys.c"},
    Syscall{number: 124, name: "getsid", file: "sys_getsid", sysname: "kernel/sys.c"},
    Syscall{number: 125, name: "capget", file: "sys_capget", sysname: "kernel/capability.c"},
    Syscall{number: 126, name: "capset", file: "sys_capset", sysname: "kernel/capability.c"},
    Syscall{number: 127, name: "rt_sigpending", file: "sys_rt_sigpending", sysname: "kernel/signal.c"},
    Syscall{number: 128, name: "rt_sigtimedwait", file: "sys_rt_sigtimedwait", sysname: "kernel/signal.c"},
    Syscall{number: 129, name: "rt_sigqueueinfo", file: "sys_rt_sigqueueinfo", sysname: "kernel/signal.c"},
    Syscall{number: 269, name: "faccessat", file: "sys_faccessat", sysname: "fs/open.c"},
    Syscall{number: 268, name: "fchmodat", file: "sys_fchmodat", sysname: "fs/open.c"},
    Syscall{number: 167, name: "swapon", file: "sys_swapon", sysname: "mm/swapfile.c"},
    Syscall{number: 118, name: "getresuid", file: "sys_getresuid", sysname: "kernel/sys.c"},
    Syscall{number: 59, name: "execve", file: "stub_execve", sysname: "fs/exec.c"},
    Syscall{number: 58, name: "vfork", file: "stub_vfork", sysname: "kernel/fork.c"},
    Syscall{number: 55, name: "getsockopt", file: "sys_getsockopt", sysname: "net/socket.c"},
    Syscall{number: 54, name: "setsockopt", file: "sys_setsockopt", sysname: "net/socket.c"},
    Syscall{number: 57, name: "fork", file: "stub_fork", sysname: "kernel/fork.c"},
    Syscall{number: 56, name: "clone", file: "stub_clone", sysname: "kernel/fork.c"},
    Syscall{number: 51, name: "getsockname", file: "sys_getsockname", sysname: "net/socket.c"},
    Syscall{number: 50, name: "listen", file: "sys_listen", sysname: "net/socket.c"},
    Syscall{number: 53, name: "socketpair", file: "sys_socketpair", sysname: "net/socket.c"},
    Syscall{number: 52, name: "getpeername", file: "sys_getpeername", sysname: "net/socket.c"},
    Syscall{number: 259, name: "mknodat", file: "sys_mknodat", sysname: "fs/namei.c"},
    Syscall{number: 312, name: "kcmp", file: "sys_kcmp", sysname: "kernel/kcmp.c"},
    Syscall{number: 298, name: "perf_event_open", file: "sys_perf_event_open", sysname: "kernel/events/core.c"},
    Syscall{number: 299, name: "recvmmsg", file: "sys_recvmmsg", sysname: "net/socket.c"},
    Syscall{number: 296, name: "pwritev", file: "sys_pwritev", sysname: "fs/read_write.c"},
    Syscall{number: 297, name: "rt_tgsigqueueinfo", file: "sys_rt_tgsigqueueinfo", sysname: "kernel/signal.c"},
    Syscall{number: 294, name: "inotify_init1", file: "sys_inotify_init1", sysname: "fs/notify/inotify/inotify_user.c"},
    Syscall{number: 295, name: "preadv", file: "sys_preadv", sysname: "fs/read_write.c"},
    Syscall{number: 292, name: "dup3", file: "sys_dup3", sysname: "fs/file.c"},
    Syscall{number: 293, name: "pipe2", file: "sys_pipe2", sysname: "fs/pipe.c"},
    Syscall{number: 290, name: "eventfd2", file: "sys_eventfd2", sysname: "fs/eventfd.c"},
    Syscall{number: 291, name: "epoll_create1", file: "sys_epoll_create1", sysname: "fs/eventpoll.c"},
    Syscall{number: 164, name: "settimeofday", file: "sys_settimeofday", sysname: "kernel/time.c"},
    Syscall{number: 201, name: "time", file: "sys_time", sysname: "kernel/time.c"},
    Syscall{number: 199, name: "fremovexattr", file: "sys_fremovexattr", sysname: "fs/xattr.c"},
    Syscall{number: 179, name: "quotactl", file: "sys_quotactl", sysname: "fs/quota/quota.c"},
    Syscall{number: 200, name: "tkill", file: "sys_tkill", sysname: "kernel/signal.c"},
    Syscall{number: 195, name: "llistxattr", file: "sys_llistxattr", sysname: "fs/xattr.c"},
    Syscall{number: 194, name: "listxattr", file: "sys_listxattr", sysname: "fs/xattr.c"},
    Syscall{number: 197, name: "removexattr", file: "sys_removexattr", sysname: "fs/xattr.c"},
    Syscall{number: 178, name: "query_module", file: "None", sysname: "None"},
    Syscall{number: 191, name: "getxattr", file: "sys_getxattr", sysname: "fs/xattr.c"},
    Syscall{number: 190, name: "fsetxattr", file: "sys_fsetxattr", sysname: "fs/xattr.c"},
    Syscall{number: 193, name: "fgetxattr", file: "sys_fgetxattr", sysname: "fs/xattr.c"},
    Syscall{number: 192, name: "lgetxattr", file: "sys_lgetxattr", sysname: "fs/xattr.c"},
    Syscall{number: 115, name: "getgroups", file: "sys_getgroups", sysname: "kernel/groups.c"},
    Syscall{number: 114, name: "setregid", file: "sys_setregid", sysname: "kernel/sys.c"},
    Syscall{number: 88, name: "symlink", file: "sys_symlink", sysname: "fs/namei.c"},
    Syscall{number: 89, name: "readlink", file: "sys_readlink", sysname: "fs/stat.c"},
    Syscall{number: 111, name: "getpgrp", file: "sys_getpgrp", sysname: "kernel/sys.c"},
    Syscall{number: 110, name: "getppid", file: "sys_getppid", sysname: "kernel/sys.c"},
    Syscall{number: 113, name: "setreuid", file: "sys_setreuid", sysname: "kernel/sys.c"},
    Syscall{number: 112, name: "setsid", file: "sys_setsid", sysname: "kernel/sys.c"},
    Syscall{number: 82, name: "rename", file: "sys_rename", sysname: "fs/namei.c"},
    Syscall{number: 83, name: "mkdir", file: "sys_mkdir", sysname: "fs/namei.c"},
    Syscall{number: 80, name: "chdir", file: "sys_chdir", sysname: "fs/open.c"},
    Syscall{number: 81, name: "fchdir", file: "sys_fchdir", sysname: "fs/open.c"},
    Syscall{number: 86, name: "link", file: "sys_link", sysname: "fs/namei.c"},
    Syscall{number: 87, name: "unlink", file: "sys_unlink", sysname: "fs/namei.c"},
    Syscall{number: 84, name: "rmdir", file: "sys_rmdir", sysname: "fs/namei.c"},
    Syscall{number: 85, name: "creat", file: "sys_creat", sysname: "fs/open.c"},
    Syscall{number: 251, name: "ioprio_set", file: "sys_ioprio_set", sysname: "fs/ioprio.c"},
    Syscall{number: 304, name: "open_by_handle_at", file: "sys_open_by_handle_at", sysname: "fs/fhandle.c"},
    Syscall{number: 198, name: "lremovexattr", file: "sys_lremovexattr", sysname: "fs/xattr.c"},
    Syscall{number: 256, name: "migrate_pages", file: "sys_migrate_pages", sysname: "mm/mempolicy.c"},
    Syscall{number: 206, name: "io_setup", file: "sys_io_setup", sysname: "fs/aio.c"},
    Syscall{number: 226, name: "timer_delete", file: "sys_timer_delete", sysname: "kernel/posix-timers.c"},
    Syscall{number: 257, name: "openat", file: "sys_openat", sysname: "fs/open.c"},
    Syscall{number: 3, name: "close", file: "sys_close", sysname: "fs/open.c"},
    Syscall{number: 177, name: "get_kernel_syms", file: "None", sysname: "None"},
    Syscall{number: 254, name: "inotify_add_watch", file: "sys_inotify_add_watch", sysname: "fs/notify/inotify/inotify_user.c"},
    Syscall{number: 7, name: "poll", file: "sys_poll", sysname: "fs/select.c"},
    Syscall{number: 247, name: "waitid", file: "sys_waitid", sysname: "kernel/exit.c"},
    Syscall{number: 273, name: "set_robust_list", file: "sys_set_robust_list", sysname: "kernel/futex.c"},
    Syscall{number: 255, name: "inotify_rm_watch", file: "sys_inotify_rm_watch", sysname: "fs/notify/inotify/inotify_user.c"},
    Syscall{number: 308, name: "setns", file: "sys_setns", sysname: "kernel/nsproxy.c"},
    Syscall{number: 309, name: "getcpu", file: "sys_getcpu", sysname: "kernel/sys.c"},
    Syscall{number: 300, name: "fanotify_init", file: "sys_fanotify_init", sysname: "fs/notify/fanotify/fanotify_user.c"},
    Syscall{number: 301, name: "fanotify_mark", file: "sys_fanotify_mark", sysname: "fs/notify/fanotify/fanotify_user.c"},
    Syscall{number: 302, name: "prlimit64", file: "sys_prlimit64", sysname: "kernel/sys.c"},
    Syscall{number: 303, name: "name_to_handle_at", file: "sys_name_to_handle_at", sysname: "fs/fhandle.c"},
    Syscall{number: 225, name: "timer_getoverrun", file: "sys_timer_getoverrun", sysname: "kernel/posix-timers.c"},
    Syscall{number: 305, name: "clock_adjtime", file: "sys_clock_adjtime", sysname: "kernel/posix-timers.c"},
    Syscall{number: 306, name: "syncfs", file: "sys_syncfs", sysname: "fs/sync.c"},
    Syscall{number: 307, name: "sendmmsg", file: "sys_sendmmsg", sysname: "net/socket.c"},
    Syscall{number: 245, name: "mq_getsetattr", file: "sys_mq_getsetattr", sysname: "ipc/mqueue.c"},
    Syscall{number: 244, name: "mq_notify", file: "sys_mq_notify", sysname: "ipc/mqueue.c"},
    Syscall{number: 108, name: "getegid", file: "sys_getegid", sysname: "kernel/sys.c"},
    Syscall{number: 109, name: "setpgid", file: "sys_setpgid", sysname: "kernel/sys.c"},
    Syscall{number: 241, name: "mq_unlink", file: "sys_mq_unlink", sysname: "ipc/mqueue.c"},
    Syscall{number: 240, name: "mq_open", file: "sys_mq_open", sysname: "ipc/mqueue.c"},
    Syscall{number: 243, name: "mq_timedreceive", file: "sys_mq_timedreceive", sysname: "ipc/mqueue.c"},
    Syscall{number: 242, name: "mq_timedsend", file: "sys_mq_timedsend", sysname: "ipc/mqueue.c"},
    Syscall{number: 102, name: "getuid", file: "sys_getuid", sysname: "kernel/sys.c"},
    Syscall{number: 103, name: "syslog", file: "sys_syslog", sysname: "kernel/printk/printk.c"},
    Syscall{number: 100, name: "times", file: "sys_times", sysname: "kernel/sys.c"},
    Syscall{number: 101, name: "ptrace", file: "sys_ptrace", sysname: "kernel/ptrace.c"},
    Syscall{number: 106, name: "setgid", file: "sys_setgid", sysname: "kernel/sys.c"},
    Syscall{number: 107, name: "geteuid", file: "sys_geteuid", sysname: "kernel/sys.c"},
    Syscall{number: 104, name: "getgid", file: "sys_getgid", sysname: "kernel/sys.c"},
    Syscall{number: 105, name: "setuid", file: "sys_setuid", sysname: "kernel/sys.c"},
    Syscall{number: 39, name: "getpid", file: "sys_getpid", sysname: "kernel/sys.c"},
    Syscall{number: 38, name: "setitimer", file: "sys_setitimer", sysname: "kernel/itimer.c"},
    Syscall{number: 33, name: "dup2", file: "sys_dup2", sysname: "fs/file.c"},
    Syscall{number: 32, name: "dup", file: "sys_dup", sysname: "fs/file.c"},
    Syscall{number: 31, name: "shmctl", file: "sys_shmctl", sysname: "ipc/shm.c"},
    Syscall{number: 30, name: "shmat", file: "sys_shmat", sysname: "ipc/shm.c"},
    Syscall{number: 37, name: "alarm", file: "sys_alarm", sysname: "kernel/timer.c"},
    Syscall{number: 36, name: "getitimer", file: "sys_getitimer", sysname: "kernel/itimer.c"},
    Syscall{number: 35, name: "nanosleep", file: "sys_nanosleep", sysname: "kernel/hrtimer.c"},
    Syscall{number: 34, name: "pause", file: "sys_pause", sysname: "kernel/signal.c"},
    Syscall{number: 246, name: "kexec_load", file: "sys_kexec_load", sysname: "kernel/kexec.c"},
    Syscall{number: 310, name: "process_vm_readv", file: "sys_process_vm_readv", sysname: "mm/process_vm_access.c"},
    Syscall{number: 282, name: "signalfd", file: "sys_signalfd", sysname: "fs/signalfd.c"},
    Syscall{number: 252, name: "ioprio_get", file: "sys_ioprio_get", sysname: "fs/ioprio.c"},
    Syscall{number: 205, name: "set_thread_area", file: "None", sysname: "arch/x86/kernel/tls.c"},
    Syscall{number: 223, name: "timer_settime", file: "sys_timer_settime", sysname: "kernel/posix-timers.c"},
    Syscall{number: 176, name: "delete_module", file: "sys_delete_module", sysname: "kernel/module.c"},
    Syscall{number: 60, name: "exit", file: "sys_exit", sysname: "kernel/exit.c"},
    Syscall{number: 61, name: "wait4", file: "sys_wait4", sysname: "kernel/exit.c"},
    Syscall{number: 62, name: "kill", file: "sys_kill", sysname: "kernel/signal.c"},
    Syscall{number: 63, name: "uname", file: "sys_newuname", sysname: "kernel/sys.c"},
    Syscall{number: 64, name: "semget", file: "sys_semget", sysname: "ipc/sem.c"},
    Syscall{number: 65, name: "semop", file: "sys_semop", sysname: "ipc/sem.c"},
    Syscall{number: 66, name: "semctl", file: "sys_semctl", sysname: "ipc/sem.c"},
    Syscall{number: 67, name: "shmdt", file: "sys_shmdt", sysname: "ipc/shm.c"},
    Syscall{number: 68, name: "msgget", file: "sys_msgget", sysname: "ipc/msg.c"},
    Syscall{number: 69, name: "msgsnd", file: "sys_msgsnd", sysname: "ipc/msg.c"},
    Syscall{number: 175, name: "init_module", file: "sys_init_module", sysname: "kernel/module.c"},
    Syscall{number: 174, name: "create_module", file: "None", sysname: "None"},
    Syscall{number: 173, name: "ioperm", file: "sys_ioperm", sysname: "arch/x86/kernel/ioport.c"},
    Syscall{number: 172, name: "iopl", file: "stub_iopl", sysname: "arch/x86/kernel/ioport.c"},
    Syscall{number: 171, name: "setdomainname", file: "sys_setdomainname", sysname: "kernel/sys.c"},
    Syscall{number: 170, name: "sethostname", file: "sys_sethostname", sysname: "kernel/sys.c"},
    Syscall{number: 203, name: "sched_setaffinity", file: "sys_sched_setaffinity", sysname: "kernel/sched/core.c"},
    Syscall{number: 222, name: "timer_create", file: "sys_timer_create", sysname: "kernel/posix-timers.c"},
    Syscall{number: 288, name: "accept4", file: "sys_accept4", sysname: "net/socket.c"},
    Syscall{number: 181, name: "getpmsg", file: "None", sysname: "None"},
    Syscall{number: 253, name: "inotify_init", file: "sys_inotify_init", sysname: "fs/notify/inotify/inotify_user.c"},
    Syscall{number: 248, name: "add_key", file: "sys_add_key", sysname: "security/keys/keyctl.c"},
    Syscall{number: 182, name: "putpmsg", file: "None", sysname: "None"},
    Syscall{number: 183, name: "afs_syscall", file: "None", sysname: "None"},
    Syscall{number: 180, name: "nfsservctl", file: "None", sysname: "None"},
    Syscall{number: 2, name: "open", file: "sys_open", sysname: "fs/open.c"},
    Syscall{number: 162, name: "sync", file: "sys_sync", sysname: "fs/sync.c"},
    Syscall{number: 187, name: "readahead", file: "sys_readahead", sysname: "mm/readahead.c"},
    Syscall{number: 184, name: "tuxcall", file: "None", sysname: "None"},
    Syscall{number: 6, name: "lstat", file: "sys_newlstat", sysname: "fs/stat.c"},
    Syscall{number: 220, name: "semtimedop", file: "sys_semtimedop", sysname: "ipc/sem.c"},
    Syscall{number: 186, name: "gettid", file: "sys_gettid", sysname: "kernel/sys.c"},
    Syscall{number: 188, name: "setxattr", file: "sys_setxattr", sysname: "fs/xattr.c"},
    Syscall{number: 189, name: "lsetxattr", file: "sys_lsetxattr", sysname: "fs/xattr.c"},
    Syscall{number: 311, name: "process_vm_writev", file: "sys_process_vm_writev", sysname: "mm/process_vm_access.c"},
    Syscall{number: 202, name: "futex", file: "sys_futex", sysname: "kernel/futex.c"},
    Syscall{number: 313, name: "finit_module", file: "sys_finit_module", sysname: "kernel/module.c"},
    Syscall{number: 196, name: "flistxattr", file: "sys_flistxattr", sysname: "fs/xattr.c"},
    Syscall{number: 221, name: "fadvise64", file: "sys_fadvise64", sysname: "mm/fadvise.c"},
    Syscall{number: 185, name: "security", file: "None", sysname: "None"},
    Syscall{number: 99, name: "sysinfo", file: "sys_sysinfo", sysname: "kernel/sys.c"},
    Syscall{number: 98, name: "getrusage", file: "sys_getrusage", sysname: "kernel/sys.c"},
    Syscall{number: 168, name: "swapoff", file: "sys_swapoff", sysname: "mm/swapfile.c"},
    Syscall{number: 169, name: "reboot", file: "sys_reboot", sysname: "kernel/reboot.c"},
    Syscall{number: 229, name: "clock_getres", file: "sys_clock_getres", sysname: "kernel/posix-timers.c"},
    Syscall{number: 228, name: "clock_gettime", file: "sys_clock_gettime", sysname: "kernel/posix-timers.c"},
    Syscall{number: 91, name: "fchmod", file: "sys_fchmod", sysname: "fs/open.c"},
    Syscall{number: 90, name: "chmod", file: "sys_chmod", sysname: "fs/open.c"},
    Syscall{number: 93, name: "fchown", file: "sys_fchown", sysname: "fs/open.c"},
    Syscall{number: 92, name: "chown", file: "sys_chown", sysname: "fs/open.c"},
    Syscall{number: 95, name: "umask", file: "sys_umask", sysname: "kernel/sys.c"},
    Syscall{number: 94, name: "lchown", file: "sys_lchown", sysname: "fs/open.c"},
    Syscall{number: 97, name: "getrlimit", file: "sys_getrlimit", sysname: "kernel/sys.c"},
    Syscall{number: 96, name: "gettimeofday", file: "sys_gettimeofday", sysname: "kernel/time.c"},
    Syscall{number: 11, name: "munmap", file: "sys_munmap", sysname: "mm/mmap.c"},
    Syscall{number: 10, name: "mprotect", file: "sys_mprotect", sysname: "mm/mprotect.c"},
    Syscall{number: 13, name: "rt_sigaction", file: "sys_rt_sigaction", sysname: "kernel/signal.c"},
    Syscall{number: 12, name: "brk", file: "sys_brk", sysname: "mm/mmap.c"},
    Syscall{number: 15, name: "rt_sigreturn", file: "stub_rt_sigreturn", sysname: "arch/x86/kernel/signal.c"},
    Syscall{number: 14, name: "rt_sigprocmask", file: "sys_rt_sigprocmask", sysname: "kernel/signal.c"},
    Syscall{number: 17, name: "pread64", file: "sys_pread64", sysname: "fs/read_write.c"},
    Syscall{number: 16, name: "ioctl", file: "sys_ioctl", sysname: "fs/ioctl.c"},
    Syscall{number: 19, name: "readv", file: "sys_readv", sysname: "fs/read_write.c"},
    Syscall{number: 18, name: "pwrite64", file: "sys_pwrite64", sysname: "fs/read_write.c"},
    Syscall{number: 117, name: "setresuid", file: "sys_setresuid", sysname: "kernel/sys.c"},
    Syscall{number: 250, name: "keyctl", file: "sys_keyctl", sysname: "security/keys/keyctl.c"},
    Syscall{number: 116, name: "setgroups", file: "sys_setgroups", sysname: "kernel/groups.c"},
    Syscall{number: 274, name: "get_robust_list", file: "sys_get_robust_list", sysname: "kernel/futex.c"},
    Syscall{number: 204, name: "sched_getaffinity", file: "sys_sched_getaffinity", sysname: "kernel/sched/core.c"},
    Syscall{number: 275, name: "splice", file: "sys_splice", sysname: "fs/splice.c"},
    Syscall{number: 151, name: "mlockall", file: "sys_mlockall", sysname: "mm/mlock.c"},
    Syscall{number: 150, name: "munlock", file: "sys_munlock", sysname: "mm/mlock.c"},
    Syscall{number: 153, name: "vhangup", file: "sys_vhangup", sysname: "fs/open.c"},
    Syscall{number: 152, name: "munlockall", file: "sys_munlockall", sysname: "mm/mlock.c"},
    Syscall{number: 155, name: "pivot_root", file: "sys_pivot_root", sysname: "fs/namespace.c"},
    Syscall{number: 154, name: "modify_ldt", file: "sys_modify_ldt", sysname: "arch/x86/um/ldt.c"},
    Syscall{number: 157, name: "prctl", file: "sys_prctl", sysname: "kernel/sys.c"},
    Syscall{number: 156, name: "_sysctl", file: "sys_sysctl", sysname: "kernel/sysctl_binary.c"},
    Syscall{number: 159, name: "adjtimex", file: "sys_adjtimex", sysname: "kernel/time.c"},
    Syscall{number: 158, name: "arch_prctl", file: "sys_arch_prctl", sysname: "arch/x86/um/syscalls_64.c"},
    Syscall{number: 277, name: "sync_file_range", file: "sys_sync_file_range", sysname: "fs/sync.c"},
    Syscall{number: 234, name: "tgkill", file: "sys_tgkill", sysname: "kernel/signal.c"},
    Syscall{number: 238, name: "set_mempolicy", file: "sys_set_mempolicy", sysname: "mm/mempolicy.c"},
    Syscall{number: 239, name: "get_mempolicy", file: "sys_get_mempolicy", sysname: "mm/mempolicy.c"},
    Syscall{number: 279, name: "move_pages", file: "sys_move_pages", sysname: "mm/migrate.c"},
    Syscall{number: 207, name: "io_destroy", file: "sys_io_destroy", sysname: "fs/aio.c"},
    Syscall{number: 235, name: "utimes", file: "sys_utimes", sysname: "fs/utimes.c"},
    Syscall{number: 236, name: "vserver", file: "None", sysname: "None"},
    Syscall{number: 237, name: "mbind", file: "sys_mbind", sysname: "mm/mempolicy.c"},
    Syscall{number: 230, name: "clock_nanosleep", file: "sys_clock_nanosleep", sysname: "kernel/posix-timers.c"},
    Syscall{number: 231, name: "exit_group", file: "sys_exit_group", sysname: "kernel/exit.c"},
    Syscall{number: 232, name: "epoll_wait", file: "sys_epoll_wait", sysname: "fs/eventpoll.c"},
    Syscall{number: 233, name: "epoll_ctl", file: "sys_epoll_ctl", sysname: "fs/eventpoll.c"},
    Syscall{number: 224, name: "timer_gettime", file: "sys_timer_gettime", sysname: "kernel/posix-timers.c"},
    Syscall{number: 280, name: "utimensat", file: "sys_utimensat", sysname: "fs/utimes.c"},
    Syscall{number: 48, name: "shutdown", file: "sys_shutdown", sysname: "net/socket.c"},
    Syscall{number: 49, name: "bind", file: "sys_bind", sysname: "net/socket.c"},
    Syscall{number: 46, name: "sendmsg", file: "sys_sendmsg", sysname: "net/socket.c"},
    Syscall{number: 47, name: "recvmsg", file: "sys_recvmsg", sysname: "net/socket.c"},
    Syscall{number: 44, name: "sendto", file: "sys_sendto", sysname: "net/socket.c"},
    Syscall{number: 45, name: "recvfrom", file: "sys_recvfrom", sysname: "net/socket.c"},
    Syscall{number: 42, name: "connect", file: "sys_connect", sysname: "net/socket.c"},
    Syscall{number: 43, name: "accept", file: "sys_accept", sysname: "net/socket.c"},
    Syscall{number: 40, name: "sendfile", file: "sys_sendfile64", sysname: "fs/read_write.c"},
    Syscall{number: 41, name: "socket", file: "sys_socket", sysname: "net/socket.c"},
    Syscall{number: 1, name: "write", file: "sys_write", sysname: "fs/read_write.c"},
    Syscall{number: 5, name: "fstat", file: "sys_newfstat", sysname: "fs/stat.c"},
    Syscall{number: 9, name: "mmap", file: "sys_mmap", sysname: "arch/x86/kernel/sys_x86_64.c"},
    Syscall{number: 272, name: "unshare", file: "sys_unshare", sysname: "kernel/fork.c"},
    Syscall{number: 146, name: "sched_get_priority_max", file: "sys_sched_get_priority_max", sysname: "kernel/sched/core.c"},
    Syscall{number: 147, name: "sched_get_priority_min", file: "sys_sched_get_priority_min", sysname: "kernel/sched/core.c"},
    Syscall{number: 144, name: "sched_setscheduler", file: "sys_sched_setscheduler", sysname: "kernel/sched/core.c"},
    Syscall{number: 145, name: "sched_getscheduler", file: "sys_sched_getscheduler", sysname: "kernel/sched/core.c"},
    Syscall{number: 142, name: "sched_setparam", file: "sys_sched_setparam", sysname: "kernel/sched/core.c"},
    Syscall{number: 143, name: "sched_getparam", file: "sys_sched_getparam", sysname: "kernel/sched/core.c"},
    Syscall{number: 140, name: "getpriority", file: "sys_getpriority", sysname: "kernel/sys.c"},
    Syscall{number: 141, name: "setpriority", file: "sys_setpriority", sysname: "kernel/sys.c"},
    Syscall{number: 209, name: "io_submit", file: "sys_io_submit", sysname: "fs/aio.c"},
    Syscall{number: 208, name: "io_getevents", file: "sys_io_getevents", sysname: "fs/aio.c"},
    Syscall{number: 148, name: "sched_rr_get_interval", file: "sys_sched_rr_get_interval", sysname: "kernel/sched/core.c"},
    Syscall{number: 149, name: "mlock", file: "sys_mlock", sysname: "mm/mlock.c"},
    Syscall{number: 77, name: "ftruncate", file: "sys_ftruncate", sysname: "fs/open.c"},
    Syscall{number: 76, name: "truncate", file: "sys_truncate", sysname: "fs/open.c"},
    Syscall{number: 75, name: "fdatasync", file: "sys_fdatasync", sysname: "fs/sync.c"},
    Syscall{number: 74, name: "fsync", file: "sys_fsync", sysname: "fs/sync.c"},
    Syscall{number: 73, name: "flock", file: "sys_flock", sysname: "fs/locks.c"},
    Syscall{number: 72, name: "fcntl", file: "sys_fcntl", sysname: "fs/fcntl.c"},
    Syscall{number: 71, name: "msgctl", file: "sys_msgctl", sysname: "ipc/msg.c"},
    Syscall{number: 70, name: "msgrcv", file: "sys_msgrcv", sysname: "ipc/msg.c"},
    Syscall{number: 278, name: "vmsplice", file: "sys_vmsplice", sysname: "fs/splice.c"},
    Syscall{number: 79, name: "getcwd", file: "sys_getcwd", sysname: "fs/dcache.c"},
    Syscall{number: 78, name: "getdents", file: "sys_getdents", sysname: "fs/readdir.c"},
    Syscall{number: 263, name: "unlinkat", file: "sys_unlinkat", sysname: "fs/namei.c"},
    Syscall{number: 249, name: "request_key", file: "sys_request_key", sysname: "security/keys/keyctl.c"},
    Syscall{number: 262, name: "newfstatat", file: "sys_newfstatat", sysname: "fs/stat.c"},
    Syscall{number: 227, name: "clock_settime", file: "sys_clock_settime", sysname: "kernel/posix-timers.c"},
    Syscall{number: 261, name: "futimesat", file: "sys_futimesat", sysname: "fs/utimes.c"},
    Syscall{number: 270, name: "pselect6", file: "sys_pselect6", sysname: "fs/select.c"},
    Syscall{number: 260, name: "fchownat", file: "sys_fchownat", sysname: "fs/open.c"},
    Syscall{number: 276, name: "tee", file: "sys_tee", sysname: "fs/splice.c"},
    Syscall{number: 267, name: "readlinkat", file: "sys_readlinkat", sysname: "fs/stat.c"},
    Syscall{number: 266, name: "symlinkat", file: "sys_symlinkat", sysname: "fs/namei.c"},
];
