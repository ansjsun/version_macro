#### example 

````

#[cfg(not(debug_assertions))]
mod version {
    use version_macro::{build_git_branch, build_git_version, build_time};

    pub const BUILD_TIME: &str = build_time!();
    pub const BUILD_GIT_BRANCH: &str = build_git_branch!();
    pub const BUILD_GIT_VERSION: &str = build_git_version!();

    pub fn version() -> String {
        format!(
            "lark build info: [git_branch:{} , build_time:{} , git_version:{}]",
            BUILD_GIT_BRANCH, BUILD_TIME, BUILD_GIT_VERSION
        )
    }
}

#[cfg(debug_assertions)]
mod version {
    pub fn version() -> String {
        format!(
            "lark build info: [git_branch:{} , build_time:{} , git_version:{}]",
            "debug", "debug", "debug"
        )
    }
}


````