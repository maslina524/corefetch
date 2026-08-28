use crate::{
    info,
    abort,
    windows::https::Request,
    windows::fs::{self, File, Access},
    windows::path::Path,
    windows::link::{HMODULE, LoadLibraryW, FreeLibrary},
    sync::OnceLock
};

static LUA: OnceLock<LuaLib> = OnceLock::new();

const LUA_DOWNLOAD_URL: &str = "https://raw.githubusercontent.com/maslina524/corefetch/refs/heads/main/src/bin/lua55.dll";

pub struct LuaLib {
    handle: HMODULE
}

// SAFETY: THE STRUCTURE IS NOT THREAD-SAFE;
// We are not going to modify non-thread-safe fields,
// using the structure from different threads will not cause problems.
unsafe impl Sync for LuaLib {}

impl LuaLib {
    pub fn get() -> &'static Self {
        LUA.get_or_init(|| {
            // Load library
            let lib = load();

            Self { handle: lib }
        })
    }
}

pub fn get_lua_path() -> Path {
    let dir = Path::corefetch().join("bin");
    let path = dir.join("lua55.dll");
    if !path.exists() {
        info!("Downloading lua dll");

        let resp = Request::new(LUA_DOWNLOAD_URL).unwrap().get();
        if !resp.is_success() {
            abort!("Failed to download `lua55.dll`: {}", resp.code());
        }

        if let Err(e) = fs::create_dirs(dir) {
            abort!("Failed to write data to `lua55.dll`: {e}");
        }

        let content = resp.into_content();
        let file = match File::create_always(&path, Access::Write) {
            Ok(f) => f,
            Err(e) => abort!("Failed to create new file `lua55.dll`: {e}")
        };
        
        if let Err(e) = file.write(content) {
            abort!("Failed to write data to `lua55.dll`: {e}");
        }
    }

    path
}

fn load() -> HMODULE {
    // SAFETY: An ASCII string is always passed, everything is safe
    let lib = unsafe {
        let path = get_lua_path().as_wide_str().unwrap();
        LoadLibraryW(path.as_ptr())
    };
    if lib.is_null() {
        abort!("Failed to load nvml.dll");
    }
    lib
}

fn unload(lib: HMODULE) {
    // SAFETY: Completely safe
    unsafe {
        FreeLibrary(lib)
    };
}

#[cfg(test)]
mod tests {
    use crate::lua::get_lua_path;

    #[test]
    fn get_lua_path_test() {
        let path = get_lua_path();
        println!("{path}");
        assert!(path.exists());
    }
}