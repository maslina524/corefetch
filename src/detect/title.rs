use core::{
    ptr,
    slice
};

use alloc::{
    string::String,
    vec::Vec
};

use crate::{
    format,
    logo::LogoInfo,
    os::env,
    os::path::Path,
    os::windows::{
        ConvertSidToStringSidW, GetComputerNameW, GetCurrentDirectoryW, GetCurrentProcess,
        GetCurrentProcessId, GetTokenInformation, GetUserNameExW, GetUserNameW, OpenProcessToken,
        LocalFree, CloseHandle, PSID
    },
    sync::OnceLock,
    todo_or
};

static USER_NAME : OnceLock<String> = OnceLock::new();
static HOST_NAME : OnceLock<String> = OnceLock::new();

const UNLEN      : usize            = 256;
const PCNAME_LEN : usize            = 15;
const TOKEN_QUERY: u32              = 8;

#[repr(C)]
#[derive(Clone)]
struct SID_AND_ATTRIBUTES {
    pub sid: PSID,
    pub attributes: u32,
}

#[repr(C)]
#[derive(Clone)]
struct TOKEN_USER {
    pub user: SID_AND_ATTRIBUTES,
}

pub fn user_name() -> &'static String {
    USER_NAME.get_or_init(|| {
        let mut buf = [0u16; UNLEN + 1];
        let mut size = (UNLEN + 1) as u32;
        
        // SAFETY: Completely safe
        let ret = unsafe {
            GetUserNameW(
                (&raw mut buf).cast(),
                &raw mut size
            )
        };
        if ret == 0 {
            return String::new();
        }

        String::from_utf16_lossy(&buf).rsplit('\0').collect()
    })
}

pub fn colored_user_name() -> String {
    format!(
        "\x1b[{}m{}\x1b[0m",
        LogoInfo::get().unwrap().color_title,
        user_name()
    )
}

pub fn host_name() -> &'static String {
    HOST_NAME.get_or_init(|| {
        let mut buf = [0u16; PCNAME_LEN + 1];
        let mut size = (UNLEN + 1) as u32;
        
        // SAFETY: Completely safe
        let ret = unsafe {
            GetComputerNameW(
                (&raw mut buf).cast(),
                &raw mut size,
            )
        };
        if ret == 0 {
            return String::new();
        }

        String::from_utf16_lossy(&buf).to_lowercase().rsplit('\0').collect()
    })
}

pub fn colored_host_name() -> String {
    format!(
        "\x1b[{}m{}\x1b[0m",
        LogoInfo::get().unwrap().color_title,
        host_name()
    )
}

pub fn full_user_name() -> String {
    let mut buf = [0u16; UNLEN + 1];
    let mut size = (UNLEN + 1) as u32;
    
    // SAFETY: Completely safe
    let ret = unsafe {
        GetUserNameExW(
            3,
            (&raw mut buf).cast(),
            &raw mut size,
        )
    };
    if !ret {
        return String::new();
    }

    String::from_utf16_lossy(&buf).rsplit('\0').collect()
}

pub fn home_dir() -> Path {
    Path::from("C:/Users").join(user_name())
}

pub fn exe_path() -> String {
    env::args()[0].clone()
}

pub const fn user_shell() -> String {
    todo_or!("Will be implemented in the future", String::new())
}

pub fn cwd() -> String {
    let mut buf = [0u16; 1024];
    let size = 1024;
    
    // SAFETY: Completely safe
    let ret = unsafe {
        GetCurrentDirectoryW(
            size,
            (&raw mut buf).cast(),
        )
    };
    if ret == 0 {
        return String::new();
    }

    String::from_utf16_lossy(&buf).rsplit('\0').collect()
}

pub fn pid() -> u32 {
    // SAFETY: Completely safe
    unsafe {
        GetCurrentProcessId()
    }
}

pub fn user_id() -> String {
    // SAFETY: Completely safe
    let cur_proc = unsafe { GetCurrentProcess() };
    let mut token  = ptr::null_mut();

    // SAFETY: Completely safe
    let ret = unsafe {
        OpenProcessToken(
            cur_proc, 
            TOKEN_QUERY, 
            &raw mut token
        )
    };
    if ret == 0 {
        return String::new();
    }

    let mut size = 0;
    // SAFETY: Completely safe
    unsafe {
        GetTokenInformation(
            token, 
            1, 
            ptr::null_mut(), 
            0, 
            &raw mut size
        ) 
    };
    if size == 0 {
        // SAFETY: Completely safe
        unsafe { CloseHandle(token) };
        return String::new();
    }

    let mut buffer: Vec<TOKEN_USER> = Vec::with_capacity(size as usize);
    // SAFETY: Completely safe
    let ret = unsafe {
        GetTokenInformation(
            token,
            1,
            buffer.as_mut_ptr().cast(),
            size,
            &raw mut size,
        )
    };
    if ret == 0 {
        // SAFETY: Completely safe
        unsafe { CloseHandle(token) };
        return String::new();
    }

    let ptoken_user = buffer.as_ptr().cast::<TOKEN_USER>();
    // SAFETY: WinAPI returns a valid pointer to `TOKEN_USER`
    let token_user = unsafe {
        (*ptoken_user).clone()
    };
    let sid = token_user.user.sid;

    let mut sid_str = ptr::null_mut();
    // SAFETY: Completely safe
    let ret = unsafe {
        ConvertSidToStringSidW(
            sid,
            &raw mut sid_str
        )
    };
    if ret == 0 {
        // SAFETY: Completely safe
        unsafe { CloseHandle(token) };
        return String::new();
    }

    let result = if sid_str.is_null() {
        String::new()
    } else {
        let mut len = 0;
        // SAFETY: Up to the `\0`, this is a string allocated by WinAPI
        while unsafe { *sid_str.add(len) } != 0 {
            len += 1;
        }

        // SAFETY: Creating a string from memory allocated by WinAPI
        let slice = unsafe {
            slice::from_raw_parts(sid_str, len)
        };

        String::from_utf16_lossy(slice)
    };

    // SAFETY: Completely safe
    unsafe {
        LocalFree(sid_str.cast());
        CloseHandle(token);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use crate::detect::title;

    extern crate std;

    #[test]
    fn exe_path_test() {
        let path = title::exe_path();
        println!("{path}");
        assert!(path.contains("nofetch"));
    }

    #[test]
    fn full_user_name_test() {
        let full = title::full_user_name();
        println!("{full}");
    }

    #[test]
    fn cwd_test() {
        let cwd = title::cwd();
        println!("{cwd}");
    }

    #[test]
    fn host_test() {
        let cwd = title::host_name();
        println!("{cwd}");
    }

    #[test]
    fn user_id_test() {
        let user_id = title::user_id();
        println!("{user_id}");
    }
}