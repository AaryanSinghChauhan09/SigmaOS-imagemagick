//! Effect definitions for imagemagick.

/// Effect kind
#[derive(Debug, Clone, PartialEq)]
pub enum EffectKind {
    /// Blur effect
    Blur,
    /// Sharpen effect
    Sharpen,
    /// Edge effect
    Edge,
    /// Emboss effect
    Emboss,
    /// Oil paint effect
    OilPaint,
    /// Custom effect
    Custom(String),
}

/// Effect state
#[derive(Debug, Clone, PartialEq)]
pub enum EffectState {
    /// Idle effect
    Idle,
    /// Applied effect
    Applied,
}

/// Effect
#[derive(Debug, Clone)]
pub struct Effect {
    /// Effect ID
    pub id: String,
    /// Effect kind
    pub kind: EffectKind,
    /// Effect state
    pub state: EffectState,
    /// Effect strength (0.0-10.0)
    pub strength: f64,
    /// Effect radius
    pub radius: u32,
    /// Created timestamp
    pub created_at: u64,
}

impl Effect {
    /// Create new effect
    pub fn new(kind: EffectKind, strength: f64) -> Self {
        Self {
            id: format!("effect-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            kind,
            state: EffectState::Idle,
            strength: strength.clamp(0.0, 10.0),
            radius: 0,
            created_at: Self::now(),
        }
    }
    
    /// Set radius
    pub fn set_radius(&mut self, radius: u32) {
        self.radius = radius;
    }
    
    /// Apply effect
    pub fn apply(&mut self) {
        self.state = EffectState::Applied;
    }
    
    /// Reset effect
    pub fn reset(&mut self) {
        self.state = EffectState::Idle;
    }
    
    /// Check if applied
    pub fn is_applied(&self) -> bool {
        self.state == EffectState::Applied
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
