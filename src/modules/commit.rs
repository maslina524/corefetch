use crate::{
    impl_display_for_module,
    format_for_module,
    modules::Module, 
    sync::OnceLock
};

static COMMIT: OnceLock<Commit> = OnceLock::new();

#[derive(Debug)]
pub struct Commit {
    pub author: &'static str,
    pub email: &'static str,
    pub date: &'static str,
    pub date_small: &'static str,
    pub sha: &'static str,
    pub sha_short: &'static str,
    pub message: &'static str,
    pub files: &'static str,
    pub added: &'static str,
    pub deleted: &'static str,
    pub total: &'static str
}

impl Module for Commit {
    fn new() -> Self {
        Self {
            author: env!("COMMIT_AUTHOR"),
            email: env!("COMMIT_EMAIL"),
            date: env!("COMMIT_DATE"),
            date_small: env!("COMMIT_DATE_SMALL"),
            sha: env!("COMMIT_SHA"),
            sha_short: env!("COMMIT_SHA_SMALL"),
            message: env!("COMMIT_MESSAGE"),
            files: env!("COMMIT_FILES"),
            added: env!("COMMIT_ADDED"),
            deleted: env!("COMMIT_DELETED"),
            total: env!("COMMIT_TOTAL")
        }
    }

    fn get() -> &'static Self {
        COMMIT.get_or_init(|| {
            Self::new()
        })
    }

    fn key(&self) -> &'static str {
        "Commit"
    }

    fn title(&self) -> &'static str {
        "{message} [{date-small}] @ \x1b[32m+{added} \x1b[31m-{deleted}\x1b[0m"
    }

    fn string_name(&self) -> &'static str {
        "commit"
    }

    format_for_module!(
        Commit,
        author, email, date, date_small,
        sha, sha_short, message, files, added,
        deleted, total
    );
}

impl_display_for_module!(Commit);