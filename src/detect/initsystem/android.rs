use alloc::{
    borrow::ToOwned
};

use crate::{
    detect::initsystem::InitSystemInfo, 
    linux::path::Path
};

impl InitSystemInfo {
    #[todo::todo("Too time-consuming to implement for every distribution and system")]
    pub fn new() -> Self {
        let name = "Unknown".to_owned();
        let exe = Path::from("/init");
        let version = "0.0.0.0".to_owned();

        Self { 
            exe, 
            pid: 1, 
            name, 
            version
        }
    }
}