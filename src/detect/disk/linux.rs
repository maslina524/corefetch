use core::ffi::{CStr, c_char};

use alloc::{
    vec::Vec,
    string::String
};

use crate::{
    formats::{MemorySize, Percent}, 
    linux::{
        error::ErrorCode, 
        libc::{
            Stat, Statvfs, Tm, getmntent, localtime_r, setmntent, stat, statvfs, strftime
        }
    }, 
    modules::disk::Disk, 
    warning
};

const ST_RDONLY: u64 = 0b1;

static SKIP_TYPES: [&CStr; 25] = [
    c"proc", c"sysfs", c"tmpfs", c"devtmpfs", c"devpts",
    c"cgroup", c"cgroup2", c"pstore", c"securityfs",
    c"debugfs", c"tracefs", c"fusectl", c"mqueue",
    c"hugetlbfs", c"bpf", c"configfs", c"autofs",
    c"binfmt_misc", c"rpc_pipefs", c"nsfs", c"overlay",
    c"squashfs", c"ramfs", c"fuse.gvfsd-fuse", c"fuse.portal"
];

static SKIP_DIRS: [&CStr; 9] = [
    c"/proc", c"/sys", c"/dev", c"/run", c"/tmp",
    c"/var/lib/docker", c"/var/lib/containers",
    c"/snap", c"/boot/efi"
];

fn is_skipped(typ: &CStr) -> bool {
    SKIP_TYPES.contains(&typ) || SKIP_DIRS.contains(&typ)
}

pub fn get_disks_linux() -> Vec<Disk> {
    let fp = setmntent(c"/proc/mounts".as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        warning!("Failed to read /proc/mounts: {}", ErrorCode::last());
        return Vec::new();
    }

    let mut ret = Vec::with_capacity(8);
    let mut ent;
    loop {
        ent = getmntent(fp);
        if ent.is_null() {
            break;
        }

        let data = unsafe { &*ent };
        let fs_cstr = unsafe { CStr::from_ptr(data.mnt_typeL) };
        if is_skipped(fs_cstr) {
            continue;
        }
        
        let mount_from_cstr = unsafe { CStr::from_ptr(data.mnt_fsnameL) };
        let mount_cstr = unsafe { CStr::from_ptr(data.mnt_dirL) };

        let disk = process_mount(mount_cstr, mount_from_cstr, fs_cstr);
        ret.push(disk);
    }

    ret
}

fn process_mount(mount: &CStr, mount_from: &CStr, mount_fs: &CStr) -> Disk {
    let filesystem_str = mount_fs.to_string_lossy().into_owned();
    let mount_from_str = mount_from.to_string_lossy().into_owned();
    let mountpoint_str = mount.to_string_lossy().into_owned();

    let mut statv = Statvfs::default();
    let ret = statvfs(mount.as_ptr(), &raw mut statv);
    if ret != 0 {
        warning!(
            "Failed to call statvfs({}): {}",
            mountpoint_str,
            ErrorCode::last()
        );
    }

    let fr = if statv.f_frsize == 0 { statv.f_bsize } else { statv.f_frsize };
    let total = statv.f_blocks.saturating_mul(fr);

    let free = statv.f_bfree.saturating_mul(fr);
    let used = total.saturating_sub(free);
    let percent = (used as f64 / total as f64).clamp(0.0, 1.0);

    let is_readonly = statv.f_flag & ST_RDONLY != 0;

    let mut st = Stat::default();
    let ret = stat(mount_from.as_ptr(), &raw mut st);
    if ret != 0 {
        warning!(
            "Failed to call stat({}): {}",
            mount_from_str,
            ErrorCode::last()
        );
    }

    let mut tm = Tm::default();
    let res = localtime_r(&raw const st.st_ctime.tv_sec, &raw mut tm);
    if res.is_null() {
        warning!("localtime_r failed for {}", mount_from_str);
    }

    let mut buf = [c_char::default(); 64 + 1];
    strftime(
        buf.as_mut_ptr(),
        64,
        c"%Y-%m-%d %H:%M:%S".as_ptr(),
        &raw const tm,
    );
    let create_time = unsafe { CStr::from_ptr(buf.as_ptr()) }
        .to_string_lossy()
        .into_owned();

    Disk {
        size_used: MemorySize::from_bytes(used),
        size_total: MemorySize::from_bytes(total),
        size_percentage: Percent::new((percent * 100.0) as u8),
        files_used: 0, 
        files_total: 0,
        files_percentage: Percent::default(),
        is_external: false,
        is_hidden: false,
        filesystem: filesystem_str,
        name: String::new(),
        is_readonly,
        create_time,
        size_percentage_bar: String::new(),
        files_percentage_bar: String::new(),
        mountpoint: mountpoint_str,
        mount_from: mount_from_str,
        ..Default::default()
    }
    // Disk { 
    //     create_time: (),
    //     days: (), 
    //     hours: (), 
    //     minutes: (), 
    //     seconds: (), 
    //     milliseconds: ()
    // }
}