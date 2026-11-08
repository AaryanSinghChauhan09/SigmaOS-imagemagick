//! Imagemagick manager.

use std::collections::HashMap;
use crate::image::Image;
use crate::operation::{Operation, OperationKind, OperationResult};
use crate::format::{ImageFormat, FormatConfig};
use crate::effect::Effect;

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max images
    pub max_images: usize,
    /// Max operations
    pub max_operations: usize,
    /// Max effects per image
    pub max_effects_per_image: usize,
    /// Default quality
    pub default_quality: u8,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_images: 50,
            max_operations: 100,
            max_effects_per_image: 10,
            default_quality: 90,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total images
    pub total_images: usize,
    /// Loaded images
    pub loaded_images: usize,
    /// Total operations
    pub total_operations: usize,
    /// Successful operations
    pub successful_operations: usize,
    /// Total effects
    pub total_effects: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Imagemagick manager
#[derive(Debug, Clone)]
pub struct ImagemagickManager {
    /// Images
    images: HashMap<String, Image>,
    /// Operations
    operations: HashMap<String, Operation>,
    /// Effects
    effects: HashMap<String, Effect>,
    /// Effect -> image mapping
    effect_image: HashMap<String, String>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for ImagemagickManager {
    fn default() -> Self {
        Self {
            images: HashMap::new(),
            operations: HashMap::new(),
            effects: HashMap::new(),
            effect_image: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_images: 0,
                loaded_images: 0,
                total_operations: 0,
                successful_operations: 0,
                total_effects: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
}

impl ImagemagickManager {
    /// Create new imagemagick manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            images: HashMap::new(),
            operations: HashMap::new(),
            effects: HashMap::new(),
            effect_image: HashMap::new(),
            options,
            statistics: ManagerStatistics {
                total_images: 0,
                loaded_images: 0,
                total_operations: 0,
                successful_operations: 0,
                total_effects: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        self.update_statistics();
    }
    
    // Image methods
    
    /// Load image
    pub fn load_image(&mut self, path: String) -> String {
        let mut image = Image::new(path);
        image.load();
        image.finish_loading(800, 600); // default size
        
        let id = image.id.clone();
        self.images.insert(id.clone(), image);
        self.update_statistics();
        id
    }
    
    /// Get image
    pub fn get_image(&self, image_id: &str) -> Option<&Image> {
        self.images.get(image_id)
    }
    
    /// Get image mutably
    pub fn get_image_mut(&mut self, image_id: &str) -> Option<&mut Image> {
        self.images.get_mut(image_id)
    }
    
    /// Get all images
    pub fn list_images(&self) -> Vec<&Image> {
        self.images.values().collect()
    }
    
    /// Remove image
    pub fn remove_image(&mut self, image_id: &str) -> bool {
        // Remove associated effects
        let effect_ids: Vec<String> = self.effect_image.iter()
            .filter(|(_, iid)| *iid == image_id)
            .map(|(eid, _)| eid.clone())
            .collect();
        
        for effect_id in effect_ids {
            self.effects.remove(&effect_id);
            self.effect_image.remove(&effect_id);
        }
        
        if self.images.remove(image_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Set image size
    pub fn set_image_size(&mut self, image_id: &str, width: u32, height: u32) -> bool {
        if let Some(image) = self.images.get_mut(image_id) {
            image.set_size(width, height);
            return true;
        }
        false
    }
    
    /// Get images by state
    pub fn get_images_by_state(&self, state: &crate::image::ImageState) -> Vec<&Image> {
        self.images.values()
            .filter(|i| i.state == *state)
            .collect()
    }
    
    /// Get loaded images
    pub fn get_loaded_images(&self) -> Vec<&Image> {
        self.get_images_by_state(&crate::image::ImageState::Loaded)
    }
    
    /// Search images
    pub fn search_images(&self, query: &str) -> Vec<&Image> {
        let query_lower = query.to_lowercase();
        self.images.values()
            .filter(|i| i.name.to_lowercase().contains(&query_lower) ||
                       i.path.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Operation methods
    
    /// Create operation
    pub fn create_operation(&mut self, kind: OperationKind) -> String {
        let op = Operation::new(kind);
        let id = op.id.clone();
        self.operations.insert(id.clone(), op);
        self.update_statistics();
        id
    }
    
    /// Get operation
    pub fn get_operation(&self, op_id: &str) -> Option<&Operation> {
        self.operations.get(op_id)
    }
    
    /// Get operation mutably
    pub fn get_operation_mut(&mut self, op_id: &str) -> Option<&mut Operation> {
        self.operations.get_mut(op_id)
    }
    
    /// Get all operations
    pub fn list_operations(&self) -> Vec<&Operation> {
        self.operations.values().collect()
    }
    
    /// Remove operation
    pub fn remove_operation(&mut self, op_id: &str) -> bool {
        if self.operations.remove(op_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Resize image
    pub fn resize_image(&mut self, image_id: &str, width: u32, height: u32) -> String {
        let mut op = Operation::new(OperationKind::Resize);
        op.add_input(image_id.to_string());
        op.add_arg(width.to_string());
        op.add_arg(height.to_string());
        
        let id = op.id.clone();
        op.start();
        op.finish(OperationResult::success(format!("{}_resized", image_id), 1024));
        
        self.operations.insert(id.clone(), op);
        self.update_statistics();
        id
    }
    
    /// Convert image format
    pub fn convert_image(&mut self, image_id: &str, format: ImageFormat, output_path: String) -> String {
        let mut op = Operation::new(OperationKind::Convert);
        op.add_input(image_id.to_string());
        op.add_arg(format.get_extension().to_string());
        
        let id = op.id.clone();
        op.start();
        op.finish(OperationResult::success(output_path, 2048));
        
        self.operations.insert(id.clone(), op);
        self.update_statistics();
        id
    }
    
    /// Crop image
    pub fn crop_image(&mut self, image_id: &str, x: u32, y: u32, width: u32, height: u32) -> String {
        let mut op = Operation::new(OperationKind::Crop);
        op.add_input(image_id.to_string());
        op.add_arg(x.to_string());
        op.add_arg(y.to_string());
        op.add_arg(width.to_string());
        op.add_arg(height.to_string());
        
        let id = op.id.clone();
        op.start();
        op.finish(OperationResult::success(format!("{}_cropped", image_id), 512));
        
        self.operations.insert(id.clone(), op);
        self.update_statistics();
        id
    }
    
    /// Rotate image
    pub fn rotate_image(&mut self, image_id: &str, degrees: f64) -> String {
        let mut op = Operation::new(OperationKind::Rotate);
        op.add_input(image_id.to_string());
        op.add_arg(degrees.to_string());
        
        let id = op.id.clone();
        op.start();
        op.finish(OperationResult::success(format!("{}_rotated", image_id), 1024));
        
        self.operations.insert(id.clone(), op);
        self.update_statistics();
        id
    }
    
    /// Get operations by kind
    pub fn get_operations_by_kind(&self, kind: &OperationKind) -> Vec<&Operation> {
        self.operations.values()
            .filter(|o| o.kind == *kind)
            .collect()
    }
    
    /// Get operations by state
    pub fn get_operations_by_state(&self, state: &crate::operation::OperationState) -> Vec<&Operation> {
        self.operations.values()
            .filter(|o| o.state == *state)
            .collect()
    }
    
    /// Get successful operations
    pub fn get_successful_operations(&self) -> Vec<&Operation> {
        self.get_operations_by_state(&crate::operation::OperationState::Done)
    }
    
    // Effect methods
    
    /// Add effect
    pub fn add_effect(&mut self, image_id: &str, effect: Effect) -> String {
        let id = effect.id.clone();
        
        self.effects.insert(id.clone(), effect);
        self.effect_image.insert(id.clone(), image_id.to_string());
        self.update_statistics();
        id
    }
    
    /// Get effect
    pub fn get_effect(&self, effect_id: &str) -> Option<&Effect> {
        self.effects.get(effect_id)
    }
    
    /// Get effect mutably
    pub fn get_effect_mut(&mut self, effect_id: &str) -> Option<&mut Effect> {
        self.effects.get_mut(effect_id)
    }
    
    /// Get all effects
    pub fn list_effects(&self) -> Vec<&Effect> {
        self.effects.values().collect()
    }
    
    /// Remove effect
    pub fn remove_effect(&mut self, effect_id: &str) -> bool {
        self.effect_image.remove(effect_id);
        if self.effects.remove(effect_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Apply effect
    pub fn apply_effect(&mut self, effect_id: &str) -> bool {
        if let Some(effect) = self.effects.get_mut(effect_id) {
            effect.apply();
            return true;
        }
        false
    }
    
    /// Reset effect
    pub fn reset_effect(&mut self, effect_id: &str) -> bool {
        if let Some(effect) = self.effects.get_mut(effect_id) {
            effect.reset();
            return true;
        }
        false
    }
    
    /// Get effects for image
    pub fn get_image_effects(&self, image_id: &str) -> Vec<&Effect> {
        self.effect_image.iter()
            .filter(|(_, iid)| *iid == image_id)
            .filter_map(|(eid, _)| self.effects.get(eid))
            .collect()
    }
    
    // Statistics methods
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_images = self.images.len();
        self.statistics.loaded_images = self.images.values()
            .filter(|i| i.is_loaded())
            .count();
        self.statistics.total_operations = self.operations.len();
        self.statistics.successful_operations = self.operations.values()
            .filter(|o| o.is_done())
            .count();
        self.statistics.total_effects = self.effects.len();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            images: self.images.values().cloned().collect(),
            operations: self.operations.values().cloned().collect(),
            effects: self.effects.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.images.clear();
        for image in export.images {
            self.images.insert(image.id.clone(), image);
        }
        
        self.operations.clear();
        for op in export.operations {
            self.operations.insert(op.id.clone(), op);
        }
        
        self.effects.clear();
        for effect in export.effects {
            self.effects.insert(effect.id.clone(), effect);
        }
        
        self.update_statistics();
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Images
    pub images: Vec<Image>,
    /// Operations
    pub operations: Vec<Operation>,
    /// Effects
    pub effects: Vec<Effect>,
}
