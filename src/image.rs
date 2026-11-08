//! Image definitions for imagemagick.

/// Image kind
#[derive(Debug, Clone, PartialEq)]
pub enum ImageKind {
    /// Raster image
    Raster,
    /// Vector image
    Vector,
    /// Animated image
    Animated,
    /// Other image
    Other,
}

/// Image state
#[derive(Debug, Clone, PartialEq)]
pub enum ImageState {
    /// Idle image
    Idle,
    /// Loading image
    Loading,
    /// Loaded image
    Loaded,
    /// Error image
    Error,
}

/// Pixel
#[derive(Debug, Clone, PartialEq)]
pub struct Pixel {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
    /// Alpha component
    pub a: u8,
}

impl Pixel {
    /// Create new pixel
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    /// Create from RGB
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }
}

/// Image info
#[derive(Debug, Clone, Default)]
pub struct ImageInfo {
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
    /// Image format
    pub format: crate::format::ImageFormat,
    /// Image depth (bits per pixel)
    pub depth: u8,
    /// Image colorspace
    pub colorspace: String,
    /// Image filename
    pub filename: String,
}

impl Default for crate::format::ImageFormat {
    fn default() -> Self {
        crate::format::ImageFormat::Png
    }
}

/// Image
#[derive(Debug, Clone)]
pub struct Image {
    /// Image ID
    pub id: String,
    /// Image path
    pub path: String,
    /// Image name
    pub name: String,
    /// Image kind
    pub kind: ImageKind,
    /// Image state
    pub state: ImageState,
    /// Image info
    pub info: ImageInfo,
    /// Created timestamp
    pub created_at: u64,
    /// Last modified timestamp
    pub last_modified: u64,
}

impl Image {
    /// Create new image
    pub fn new(path: String) -> Self {
        let name = path.rsplit('/').next()
            .unwrap_or(&path)
            .to_string();
        
        Self {
            id: format!("image-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            path,
            name,
            kind: ImageKind::Raster,
            state: ImageState::Idle,
            info: ImageInfo::default(),
            created_at: Self::now(),
            last_modified: Self::now(),
        }
    }
    
    /// Set size
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.info.width = width;
        self.info.height = height;
        self.last_modified = Self::now();
    }
    
    /// Set format
    pub fn set_format(&mut self, format: crate::format::ImageFormat) {
        self.info.format = format;
        self.last_modified = Self::now();
    }
    
    /// Set depth
    pub fn set_depth(&mut self, depth: u8) {
        self.info.depth = depth;
        self.last_modified = Self::now();
    }
    
    /// Set colorspace
    pub fn set_colorspace(&mut self, colorspace: String) {
        self.info.colorspace = colorspace;
        self.last_modified = Self::now();
    }
    
    /// Load image
    pub fn load(&mut self) {
        self.state = ImageState::Loading;
        self.last_modified = Self::now();
    }
    
    /// Finish loading
    pub fn finish_loading(&mut self, width: u32, height: u32) {
        self.state = ImageState::Loaded;
        self.info.width = width;
        self.info.height = height;
        self.last_modified = Self::now();
    }
    
    /// Set error state
    pub fn set_error(&mut self) {
        self.state = ImageState::Error;
        self.last_modified = Self::now();
    }
    
    /// Get width
    pub fn get_width(&self) -> u32 {
        self.info.width
    }
    
    /// Get height
    pub fn get_height(&self) -> u32 {
        self.info.height
    }
    
    /// Get aspect ratio
    pub fn get_aspect_ratio(&self) -> f64 {
        if self.info.height == 0 {
            0.0
        } else {
            self.info.width as f64 / self.info.height as f64
        }
    }
    
    /// Get pixel count
    pub fn get_pixel_count(&self) -> u64 {
        self.info.width as u64 * self.info.height as u64
    }
    
    /// Check if loaded
    pub fn is_loaded(&self) -> bool {
        self.state == ImageState::Loaded
    }
    
    /// Check if loading
    pub fn is_loading(&self) -> bool {
        self.state == ImageState::Loading
    }
    
    /// Check if error
    pub fn is_error(&self) -> bool {
        self.state == ImageState::Error
    }
    
    /// Get image age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Get time since modification
    pub fn get_idle_seconds(&self) -> u64 {
        Self::now() - self.last_modified
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
