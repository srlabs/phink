use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum PhinkEnv {
    FuzzingWithConfig,
    FromZiggy,
    CargoManifestDir,
    AflForkServerTimeout,
    AflDebug,
    AllowList,
}

impl Display for PhinkEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PhinkEnv::FuzzingWithConfig => "PHINK_START_FUZZING_WITH_CONFIG",
            PhinkEnv::FromZiggy => "PHINK_FROM_ZIGGY",
            PhinkEnv::CargoManifestDir => "CARGO_MANIFEST_DIR",
            PhinkEnv::AflForkServerTimeout => "AFL_FORKSRV_INIT_TMOUT",
            PhinkEnv::AflDebug => "AFL_DEBUG",
            PhinkEnv::AllowList => "AFL_LLVM_ALLOWLIST",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phink_env_display() {
        assert_eq!(
            PhinkEnv::FuzzingWithConfig.to_string(),
            "PHINK_START_FUZZING_WITH_CONFIG"
        );
        assert_eq!(PhinkEnv::FromZiggy.to_string(), "PHINK_FROM_ZIGGY");
        assert_eq!(PhinkEnv::CargoManifestDir.to_string(), "CARGO_MANIFEST_DIR");
    }
}
