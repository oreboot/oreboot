
use raw_cpuid::FeatureInfo;

// https://github.com/gz/rust-cpuid/issues/26#issuecomment-785280143
pub fn amd_family_id(s: &FeatureInfo) -> u8 {
    let base_family_id = s.family_id();
    let extended_family_id = s.extended_family_id();
    if base_family_id < 0xF {
        base_family_id
    } else {
        base_family_id + extended_family_id
    }
}

// https://github.com/gz/rust-cpuid/issues/26#issuecomment-785280143
pub fn amd_model_id(s: &FeatureInfo) -> u8 {
    let base_family_id = s.family_id();
    let base_model_id = s.model_id();
    let extended_model_id = s.extended_model_id();
    if base_family_id < 0xF {
        base_model_id
    } else {
        (extended_model_id << 4) | base_model_id
    }
}
