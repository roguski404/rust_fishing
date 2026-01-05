#[derive(Clone)]
pub enum Upgrade {
    BiggerMask,
    SlowerFish,
    FasterProgress,
}

#[derive(Clone)]
pub struct UpgradeChoice {
    pub upgrade: Upgrade,
}