//! SigmaOS-imagemagick — image processing suite for SigmaOS.
//!
//! Models ImageMagick's image operations: convert, resize,
//! crop, rotate, composite, effects, formats, and pipelines.

pub mod image;
pub mod operation;
pub mod format;
pub mod effect;
pub mod manager;
pub mod options;
pub mod util;

pub use image::{Image, ImageKind, ImageState, ImageInfo, Pixel};
pub use operation::{Operation, OperationKind, OperationState, OperationResult};
pub use format::{ImageFormat, FormatConfig};
pub use effect::{Effect, EffectKind, EffectState};
pub use manager::{ImagemagickManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};
pub use util::Model;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_image_kind() {
        let kinds = vec![ImageKind::Raster, ImageKind::Vector, ImageKind::Animated, ImageKind::Other];
        assert_eq!(kinds.len(), 4);
    }
    
    #[test]
    fn test_image_state() {
        let states = vec![ImageState::Idle, ImageState::Loading, ImageState::Loaded, ImageState::Error];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_operation_kind() {
        let kinds = vec![
            OperationKind::Convert, OperationKind::Resize, OperationKind::Crop,
            OperationKind::Rotate, OperationKind::Flip, OperationKind::Flop,
            OperationKind::Composite, OperationKind::Draw, OperationKind::Annotate,
            OperationKind::Custom(String::new()),
        ];
        assert_eq!(kinds.len(), 10);
    }
    
    #[test]
    fn test_operation_state() {
        let states = vec![OperationState::Idle, OperationState::Running, OperationState::Done, OperationState::Failed];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_effect_kind() {
        let kinds = vec![EffectKind::Blur, EffectKind::Sharpen, EffectKind::Edge, EffectKind::Emboss, EffectKind::OilPaint, EffectKind::Custom(String::new())];
        assert_eq!(kinds.len(), 6);
    }
    
    #[test]
    fn test_image_creation() {
        let image = Image::new("/tmp/photo.jpg".into());
        assert_eq!(image.path, "/tmp/photo.jpg".to_string());
        assert_eq!(image.state, ImageState::Idle);
    }
    
    #[test]
    fn test_image_set_size() {
        let mut image = Image::new("img.png".into());
        image.set_size(800, 600);
        assert_eq!(image.info.width, 800);
        assert_eq!(image.info.height, 600);
    }
    
    #[test]
    fn test_image_set_format() {
        let mut image = Image::new("img.png".into());
        image.set_format(ImageFormat::Jpeg);
        assert_eq!(image.info.format, ImageFormat::Jpeg);
    }
    
    #[test]
    fn test_image_lifecycle() {
        let mut image = Image::new("img.png".into());
        image.load();
        assert_eq!(image.state, ImageState::Loading);
        image.finish_loading(800, 600);
        assert_eq!(image.state, ImageState::Loaded);
        assert_eq!(image.info.width, 800);
    }
    
    #[test]
    fn test_operation_creation() {
        let op = Operation::new(OperationKind::Resize);
        assert_eq!(op.kind, OperationKind::Resize);
        assert_eq!(op.state, OperationState::Idle);
    }
    
    #[test]
    fn test_operation_args() {
        let mut op = Operation::new(OperationKind::Resize);
        op.add_arg("800".into());
        op.add_arg("600".into());
        op.add_arg("!".into());
        
        assert_eq!(op.args.len(), 3);
        assert_eq!(op.args[0], "800".to_string());
    }
    
    #[test]
    fn test_operation_lifecycle() {
        let mut op = Operation::new(OperationKind::Convert);
        op.start();
        assert_eq!(op.state, OperationState::Running);
        op.finish(OperationResult::success("out.png".into(), 1024));
        assert_eq!(op.state, OperationState::Done);
    }
    
    #[test]
    fn test_format() {
        let formats = vec![ImageFormat::Png, ImageFormat::Jpeg, ImageFormat::Gif, ImageFormat::Bmp, ImageFormat::Tiff, ImageFormat::Webp];
        assert_eq!(formats.len(), 6);
        assert_eq!(ImageFormat::Png.get_extension(), "png");
        assert_eq!(ImageFormat::Jpeg.get_extension(), "jpg");
    }
    
    #[test]
    fn test_effect_creation() {
        let effect = Effect::new(EffectKind::Blur, 5.0);
        assert_eq!(effect.kind, EffectKind::Blur);
        assert_eq!(effect.strength, 5.0);
    }
    
    #[test]
    fn test_effect_apply() {
        let mut effect = Effect::new(EffectKind::Sharpen, 2.0);
        effect.apply();
        assert!(effect.is_applied());
        effect.reset();
        assert!(!effect.is_applied());
    }
    
    #[test]
    fn test_pixel() {
        let px = Pixel::new(255, 0, 0, 255);
        assert_eq!(px.r, 255);
        assert_eq!(px.g, 0);
        assert_eq!(px.b, 0);
        assert_eq!(px.a, 255);
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = ImagemagickManager::new();
        assert!(manager.list_images().is_empty());
        assert!(manager.list_operations().is_empty());
    }
    
    #[test]
    fn test_load_image() {
        let mut manager = ImagemagickManager::new();
        manager.initialize();
        let id = manager.load_image("/tmp/photo.jpg".into());
        assert!(manager.get_image(&id).is_some());
    }
    
    #[test]
    fn test_resize_image() {
        let mut manager = ImagemagickManager::new();
        manager.initialize();
        let id = manager.load_image("img.png".into());
        
        let op_id = manager.resize_image(&id, 400, 300);
        assert!(manager.get_operation(&op_id).is_some());
    }
    
    #[test]
    fn test_convert_format() {
        let mut manager = ImagemagickManager::new();
        manager.initialize();
        let id = manager.load_image("img.png".into());
        
        let op_id = manager.convert_image(&id, ImageFormat::Jpeg, "/tmp/out.jpg".into());
        assert!(manager.get_operation(&op_id).is_some());
    }
    
    #[test]
    fn test_add_effect() {
        let mut manager = ImagemagickManager::new();
        manager.initialize();
        let id = manager.load_image("img.png".into());
        
        let eff_id = manager.add_effect(&id, Effect::new(EffectKind::Blur, 3.0));
        assert!(manager.get_effect(&eff_id).is_some());
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = ImagemagickManager::new();
        manager.initialize();
        manager.load_image("img.png".into());
        
        let export = manager.export_data();
        
        let mut manager2 = ImagemagickManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_images().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = ImagemagickManager::new();
        manager.initialize();
        manager.load_image("a.png".into());
        manager.load_image("b.png".into());
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_images, 2);
    }
}
