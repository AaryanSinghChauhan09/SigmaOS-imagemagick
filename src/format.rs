//! Format definitions for imagemagick.

/// Image format
#[derive(Debug, Clone, PartialEq)]
pub enum ImageFormat {
    /// PNG format
    Png,
    /// JPEG format
    Jpeg,
    /// GIF format
    Gif,
    /// BMP format
    Bmp,
    /// TIFF format
    Tiff,
    /// WebP format
    Webp,
    /// SVG format
    Svg,
    /// PDF format
    Pdf,
}

impl ImageFormat {
    /// Get file extension
    pub fn get_extension(&self) -> &str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Gif => "gif",
            ImageFormat::Bmp => "bmp",
            ImageFormat::Tiff => "tiff",
            ImageFormat::Webp => "webp",
            ImageFormat::Svg => "svg",
            ImageFormat::Pdf => "pdf",
        }
    }
    
    /// Get MIME type
    pub fn get_mime_type(&self) -> &str {
        match self {
            ImageFormat::Png => "image/png",
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Gif => "image/gif",
            ImageFormat::Bmp => "image/bmp",
            ImageFormat::Tiff => "image/tiff",
            ImageFormat::Webp => "image/webp",
            ImageFormat::Svg => "image/svg+xml",
            ImageFormat::Pdf => "application/pdf",
        }
    }
    
    /// Check if lossy
    pub fn is_lossy(&self) -> bool {
        matches!(self, ImageFormat::Jpeg | ImageFormat::Webp)
    }
}

/// Format config
#[derive(Debug, Clone)]
pub struct FormatConfig {
    /// Image format
    pub format: ImageFormat,
    /// Quality (1-100)
    pub quality: u8,
    /// Compression level
    pub compression: u8,
    /// DPI
    pub dpi: u32,
}

impl FormatConfig {
    /// Create format config
    pub fn new(format: ImageFormat) -> Self {
        Self {
            format,
            quality: 90,
            compression: 6,
            dpi: 72,
        }
    }
    
    /// Set quality
    pub fn set_quality(&mut self, quality: u8) {
        self.quality = quality.clamp(1, 100);
    }
    
    /// Set compression
    pub fn set_compression(&mut self, level: u8) {
        self.compression = level.min(9);
    }
    
    /// Set DPI
    pub fn set_dpi(&mut self, dpi: u32) {
        self.dpi = dpi;
    }
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self::new(ImageFormat::Png)
    }
}
