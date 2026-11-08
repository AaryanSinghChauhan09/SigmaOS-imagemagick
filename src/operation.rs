//! Operation definitions for imagemagick.

/// Operation kind
#[derive(Debug, Clone, PartialEq)]
pub enum OperationKind {
    /// Convert operation
    Convert,
    /// Resize operation
    Resize,
    /// Crop operation
    Crop,
    /// Rotate operation
    Rotate,
    /// Flip operation
    Flip,
    /// Flop operation
    Flop,
    /// Composite operation
    Composite,
    /// Draw operation
    Draw,
    /// Annotate operation
    Annotate,
    /// Custom operation
    Custom(String),
}

/// Operation state
#[derive(Debug, Clone, PartialEq)]
pub enum OperationState {
    /// Idle operation
    Idle,
    /// Running operation
    Running,
    /// Done operation
    Done,
    /// Failed operation
    Failed,
}

/// Operation result
#[derive(Debug, Clone)]
pub struct OperationResult {
    /// Success flag
    pub success: bool,
    /// Output path
    pub output_path: Option<String>,
    /// Output size in bytes
    pub output_size: Option<u64>,
    /// Error message
    pub error: Option<String>,
    /// Created timestamp
    pub created_at: u64,
}

impl OperationResult {
    /// Create success result
    pub fn success(output_path: String, output_size: u64) -> Self {
        Self {
            success: true,
            output_path: Some(output_path),
            output_size: Some(output_size),
            error: None,
            created_at: Self::now(),
        }
    }
    
    /// Create failure result
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            output_path: None,
            output_size: None,
            error: Some(error),
            created_at: Self::now(),
        }
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Operation
#[derive(Debug, Clone)]
pub struct Operation {
    /// Operation ID
    pub id: String,
    /// Operation kind
    pub kind: OperationKind,
    /// Operation state
    pub state: OperationState,
    /// Operation arguments
    pub args: Vec<String>,
    /// Input image IDs
    pub input_image_ids: Vec<String>,
    /// Output image ID
    pub output_image_id: Option<String>,
    /// Operation result
    pub result: Option<OperationResult>,
    /// Created timestamp
    pub created_at: u64,
    /// Started timestamp
    pub started_at: Option<u64>,
    /// Finished timestamp
    pub finished_at: Option<u64>,
}

impl Operation {
    /// Create new operation
    pub fn new(kind: OperationKind) -> Self {
        Self {
            id: format!("op-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            kind,
            state: OperationState::Idle,
            args: vec![],
            input_image_ids: vec![],
            output_image_id: None,
            result: None,
            created_at: Self::now(),
            started_at: None,
            finished_at: None,
        }
    }
    
    /// Add argument
    pub fn add_arg(&mut self, arg: String) {
        self.args.push(arg);
    }
    
    /// Add input image
    pub fn add_input(&mut self, image_id: String) {
        self.input_image_ids.push(image_id);
    }
    
    /// Set output image
    pub fn set_output(&mut self, image_id: String) {
        self.output_image_id = Some(image_id);
    }
    
    /// Start operation
    pub fn start(&mut self) {
        self.state = OperationState::Running;
        self.started_at = Some(Self::now());
    }
    
    /// Finish operation
    pub fn finish(&mut self, result: OperationResult) {
        self.state = if result.success { OperationState::Done } else { OperationState::Failed };
        self.result = Some(result);
        self.finished_at = Some(Self::now());
    }
    
    /// Fail operation
    pub fn fail(&mut self, error: String) {
        self.state = OperationState::Failed;
        self.result = Some(OperationResult::failure(error));
        self.finished_at = Some(Self::now());
    }
    
    /// Check if done
    pub fn is_done(&self) -> bool {
        self.state == OperationState::Done
    }
    
    /// Check if failed
    pub fn is_failed(&self) -> bool {
        self.state == OperationState::Failed
    }
    
    /// Get operation duration in seconds
    pub fn get_duration_seconds(&self) -> u64 {
        if let (Some(start), Some(end)) = (self.started_at, self.finished_at) {
            end - start
        } else {
            0
        }
    }
    
    /// Get operation age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
