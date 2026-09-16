use core::ffi::CStr;

use alloc::vec::Vec;

use crate::{
    modules::disk::Disk, 
    linux::{
        error::ErrorCode, 
        libc::{getmntent, setmntent}
    }, 
    warning
};

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
    let filesystem = mount_fs.to_string_lossy().into_owned();
    let mount_from = mount_from.to_string_lossy().into_owned();
    let mountpoint = mount.to_string_lossy().into_owned();

    Disk {
        filesystem,
        mount_from,
        mountpoint,
        ..Default::default()
    }
}