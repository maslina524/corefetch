use core::ffi::{c_void, c_char, c_int, CStr};
use core::ptr;

use alloc::{
    string::String,
    ffi::CString,
    collections::BTreeMap
};

use crate::{
    info,
    abort,
    get_fn,
    format,
    cfg_if,
    imp::https::Request,
    imp::fs::{self, File, Access},
    imp::path::Path,
    sync::OnceLock,
    formats::capitalize
};

cfg_if! {
    if #[cfg(target_os = "windows")] {
        use crate::windows::link::{FreeLibrary, HMODULE, LoadLibraryW};

        type ApiBaseFn = unsafe extern "system" fn() -> isize;
        type LibHandle = HMODULE;
    } else if #[cfg(target_os = "linux")] {
        use crate::linux::libc::{dlopen, dlclose};

        type ApiBaseFn = *mut c_void;
        type LibHandle = *mut c_void;
    }
}

#[allow(non_camel_case_types)]
pub type lua_State = *mut c_void;

#[allow(non_camel_case_types)]
pub type luaL_newstate = unsafe extern "C" fn() -> *mut lua_State;
#[allow(non_camel_case_types)]
pub type luaL_openselectedlibs = unsafe extern "C" fn(state: *mut lua_State, mask: c_int);
#[allow(non_camel_case_types)]
pub type luaL_loadstring = unsafe extern "C" fn(state: *mut lua_State, s: *const c_char) -> c_int;
#[allow(non_camel_case_types)]
pub type lua_pcallk = unsafe extern "C" fn(
    state: *mut lua_State,
    nargs: c_int,
    nresults: c_int,
    errfunc: c_int,
    ctx: isize,
    k: Option<unsafe extern "C" fn(*mut lua_State, c_int, isize)>,
) -> c_int;
#[allow(non_camel_case_types)]
pub type lua_close = unsafe extern "C" fn(state: *mut lua_State);
#[allow(non_camel_case_types)]
pub type lua_tolstring = unsafe extern "C" fn(state: *mut lua_State, idx: c_int, len: *mut usize) -> *const c_char;
#[allow(non_camel_case_types)]
pub type lua_settop = unsafe extern "C" fn(state: *mut lua_State, idx: c_int);

static LUA: OnceLock<LuaLib> = OnceLock::new();

const LUA_DOWNLOAD_URL: &str = "https://github.com/maslina524/corefetch/raw/refs/heads/main/bin/lua55.dll";
const MODULE_GENERATOR: &str = r#"
setmetatable(Module, {
    __newindex = function(t, key, value)
        error("Attempt to modify constant", 2)
    end
})
"#;

pub enum Variable {
    String(String),
    Number(i64)
}

impl core::fmt::Debug for Variable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s:?}"),
            Self::Number(i)    => write!(f, "{i}")
        }
    }
}

pub struct LuaLib {
    handle: LibHandle,
    new_state: luaL_newstate,
    open_selected_libs: luaL_openselectedlibs,
    load_string: luaL_loadstring,
    pcall: lua_pcallk,
    close: lua_close,
    to_lstring: lua_tolstring,
}

// SAFETY: The structure is not thread-safe; however we never mutate its fields,
// and each call creates a fresh Lua state, so concurrent use from different threads
// is safe as long as the underlying Lua library is thread‑aware (it is).
unsafe impl Sync for LuaLib {}

impl LuaLib {
    pub fn get() -> &'static Self {
        LUA.get_or_init(|| {
            let lib = load();

            // SAFETY: `transmute` fully complies with the documentation
            let new_state = unsafe { get_fn!(lib, c"luaL_newstate", luaL_newstate) };
            // SAFETY: `transmute` fully complies with the documentation
            let open_selected_libs = unsafe { get_fn!(lib, c"luaL_openselectedlibs", luaL_openselectedlibs) };
            // SAFETY: `transmute` fully complies with the documentation
            let load_string = unsafe { get_fn!(lib, c"luaL_loadstring", luaL_loadstring) };
            // SAFETY: `transmute` fully complies with the documentation
            let pcall = unsafe { get_fn!(lib, c"lua_pcallk", lua_pcallk) };
            // SAFETY: `transmute` fully complies with the documentation
            let close = unsafe { get_fn!(lib, c"lua_close", lua_close) };
            // SAFETY: `transmute` fully complies with the documentation
            let to_lstring = unsafe { get_fn!(lib, c"lua_tolstring", lua_tolstring) };

            Self {
                handle: lib,
                new_state,
                open_selected_libs,
                load_string,
                pcall,
                close,
                to_lstring,
            }
        })
    }

    pub fn execute_with_vars(&self, mut code: &str, vars: BTreeMap<String, Variable>) -> String {
        code = code.trim();
        let mut ret = String::from("local Module = {{}}\n");
        for (k, v) in vars {
            ret.push_str(&format!("Module.{} = {:?}\n", capitalize(&k), v));
        }
        ret.push_str(MODULE_GENERATOR);
        ret.push_str(code);

        self.execute(&ret)
    }

    pub fn execute(&self, mut code: &str) -> String {
        code = code.trim();
        
        // SAFETY: `luaL_newstate` returns a valid state or NULL
        let state = unsafe { (self.new_state)() };
        if state.is_null() {
            abort!("Failed to create new Lua state");
        }

        // SAFETY: `luaL_openselectedlibs` takes a valid state and opens all
        // standard libraries (mask = -1 means all)
        unsafe { (self.open_selected_libs)(state, -1) };

        let c_code = CString::new(code).expect("Lua code contains NUL bytes");

        // SAFETY: `luaL_loadstring` compiles the chunk and leaves the function
        // on top of the stack. Returns 0 on success, non‑zero on error
        let load_err = unsafe { (self.load_string)(state, c_code.as_ptr()) };

        if load_err != 0 {
            // SAFETY: `lua_tolstring` returns a pointer to the error string
            let err_ptr = unsafe { (self.to_lstring)(state, -1, ptr::null_mut()) };
            let err_msg = if err_ptr.is_null() {
                String::from("unknown error (nil)")
            } else {
                // SAFETY: Lua guarantees a valid null‑terminated string
                unsafe { CStr::from_ptr(err_ptr) }
                    .to_string_lossy()
                    .into_owned()
            };
            // SAFETY: Close the state to free resources
            unsafe { (self.close)(state) };
            abort!("Lua compilation error: {}", err_msg);
        } else {
            // SAFETY: Call the compiled function with 0 arguments and expect 1 result
            // The last argument is a continuation function pointer (NULL)
            let pcall_err = unsafe { (self.pcall)(state, 0, 1, 0, 0, None) };

            if pcall_err != 0 {
                // SAFETY: `lua_tolstring` returns a pointer to the error string
                let err_ptr = unsafe { (self.to_lstring)(state, -1, ptr::null_mut()) };
                let err_msg = if err_ptr.is_null() {
                    String::from("unknown error (nil)")
                } else {
                    // SAFETY: Lua guarantees a valid null‑terminated string
                    unsafe { CStr::from_ptr(err_ptr) }
                        .to_string_lossy()
                        .into_owned()
                };
                // SAFETY: Close the state to free resources
                unsafe { (self.close)(state) };
                abort!("Lua runtime error: {}", err_msg);
            }

            // SAFETY: `lua_tolstring` returns a pointer to the string value
            let result_ptr = unsafe { (self.to_lstring)(state, -1, ptr::null_mut()) };

            let result_string = if result_ptr.is_null() {
                String::from("nil")
            } else {
                // SAFETY: Lua guarantees a valid null‑terminated string
                unsafe { CStr::from_ptr(result_ptr) }
                    .to_string_lossy()
                    .into_owned()
            };

            // SAFETY: Close the state to free resources
            unsafe { (self.close)(state) };

            result_string
        }
    }

    pub fn drop_lua() {
        if let Some(lib) = LUA.get() {
            unload(lib.handle);
        }
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
            abort!("Failed to create directory for `lua55.dll`: {e}");
        }

        let content = resp.into_content();
        let file = match File::create_always(&path, Access::Write) {
            Ok(f) => f,
            Err(e) => abort!("Failed to create file `lua55.dll`: {e}")
        };
        
        if let Err(e) = file.write(&content) {
            abort!("Failed to write data to `lua55.dll`: {e}");
        }
    }

    path
}

cfg_if! {
    if #[cfg(target_os = "windows")] {
        fn load() -> LibHandle {
            // SAFETY: `as_wide_str` returns a null‑terminated wide string,
            // which is safe to pass to `LoadLibraryW`
            let lib = unsafe {
                let path = get_lua_path().as_wide_str().unwrap();
                LoadLibraryW(path.as_ptr())
            };
            if lib.is_null() {
                abort!("Failed to load lua55.dll");
            }
            lib
        }

        fn unload(lib: LibHandle) {
            // SAFETY: The handle is guaranteed to be valid because it was
            // loaded once and never unloaded before this call
            unsafe { FreeLibrary(lib) };
        }
    } else if #[cfg(target_os = "linux")] {
        fn load() -> LibHandle {
            let path = get_lua_path().as_c_str();
            let lib = dlopen(path.as_ptr().cast(), 1);
            if lib.is_null() {
                abort!("Failed to load lua55.dll");
            }
            lib
        }

        fn unload(lib: LibHandle) {
            dlclose(lib);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lua::{LuaLib, get_lua_path};

    #[test]
    fn get_lua_path_test() {
        let path = get_lua_path();
        println!("{path}");
        assert!(path.exists());
    }

    #[test]
    fn exec_lua_code_test() {
        let lib = LuaLib::get();
        let result = lib.execute("return 2 + 2;");
        assert_eq!(result, "4");
    }
}