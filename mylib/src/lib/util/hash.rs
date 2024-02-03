use {
    once_cell::sync::Lazy,
    std::hash::BuildHasher,
    std::collections::hash_map::RandomState
};

pub static STATE: Lazy<RandomState> = Lazy::new(|| RandomState::new());

pub fn hash(value: impl std::hash::Hash) -> u64 {
    Lazy::force(&STATE).hash_one(value)
}
