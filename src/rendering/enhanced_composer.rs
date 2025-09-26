use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};
use anyhow::Result;
use std::time::{Duration, Instant};

use crate::audio::{AudioFeatures, RhythmFeatures};
use super::{WgpuContext, ShaderSystem, ShaderType, PerformanceManager, PerformanceMetrics, QualityLevel};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 1.0] },
    Vertex { position: [1.0, -1.0, 0.0], tex_coords: [1.0, 1.0] },
    Vertex { position: [1.0, 1.0, 0.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [-1.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

/// Enhanced frame composer using the new shader system architecture
pub struct EnhancedFrameComposer {
    shader_system: ShaderSystem,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    performance_manager: PerformanceManager,
    frame_start_time: Option<Instant>,
}

impl EnhancedFrameComposer {
    pub fn new(context: &WgpuContext) -> Result<Self> {
        // Initialize shader system
        let shader_system = ShaderSystem::new(&context.device, &context.config)?;

        // Create vertex buffer
        let vertex_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Enhanced Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        // Create index buffer
        let index_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Enhanced Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });

        Ok(Self {
            shader_system,
            vertex_buffer,
            index_buffer,
            performance_manager: PerformanceManager::new(60.0), // Target 60 FPS
            frame_start_time: None,
        })
    }

    /// Render a frame using the current shader with performance monitoring
    pub fn render(
        &mut self,
        context: &WgpuContext,
        audio_features: &AudioFeatures,
        rhythm_features: &RhythmFeatures,
    ) -> Result<()> {
        // Start frame timing
        let frame_start = Instant::now();
        self.frame_start_time = Some(frame_start);

        // Update shader system (handles transitions, etc.)
        self.shader_system.update(&context.device, &context.config)?;

        // Get surface texture
        let output = context.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Render using shader system with performance awareness
        let current_quality = self.performance_manager.current_quality();
        self.shader_system.render_with_quality(
            &context.device,
            &context.queue,
            &view,
            &self.vertex_buffer,
            &self.index_buffer,
            INDICES.len() as u32,
            audio_features,
            rhythm_features,
            current_quality,
        )?;

        output.present();

        // Update performance metrics
        let frame_time = frame_start.elapsed();
        let metrics = PerformanceMetrics {
            frame_time,
            cpu_time: frame_time, // Simplified - in real app would measure separately
            gpu_time: Duration::from_secs_f32(frame_time.as_secs_f32() * 0.7), // Estimate GPU portion
            fps: 1.0 / frame_time.as_secs_f32(),
            dropped_frames: if frame_time.as_millis() > 20 { 1 } else { 0 },
            memory_usage_mb: 150.0, // Estimate
        };

        let quality_changed = self.performance_manager.update(metrics);

        // Log performance adjustments
        if quality_changed {
            println!("📊 {}", self.performance_manager.performance_report());
        }

        Ok(())
    }

    /// Switch to a different shader mode
    pub fn set_shader(&mut self, shader_type: ShaderType, context: &WgpuContext) -> Result<()> {
        self.shader_system.set_shader(shader_type, &context.device, &context.config)
    }

    /// Get the currently active shader
    pub fn current_shader(&self) -> ShaderType {
        self.shader_system.current_shader()
    }

    /// Get list of available shaders
    pub fn available_shaders(&self) -> Vec<ShaderType> {
        self.shader_system.available_shaders()
    }

    /// Check if shader is currently transitioning
    pub fn is_transitioning(&self) -> bool {
        self.shader_system.is_transitioning()
    }

    /// Cycle to the next available shader
    pub fn next_shader(&mut self, context: &WgpuContext) -> Result<()> {
        let available = self.available_shaders();
        if available.is_empty() {
            return Ok(()); // No shaders available
        }

        let current = self.current_shader();
        let current_index = available.iter().position(|&s| s == current).unwrap_or(0);
        let next_index = (current_index + 1) % available.len();
        let next_shader = available[next_index];

        println!("🎨 Cycling to shader: {} -> {}", current.name(), next_shader.name());
        self.set_shader(next_shader, context)
    }

    /// Set shader based on audio characteristics (intelligent selection)
    pub fn auto_select_shader(&mut self,
                             context: &WgpuContext,
                             audio_features: &AudioFeatures,
                             rhythm_features: &RhythmFeatures) -> Result<()> {
        let current = self.current_shader();

        // Intelligent shader selection based on audio characteristics
        let recommended_shader = self.analyze_audio_for_shader(audio_features, rhythm_features);

        if recommended_shader != current {
            println!("🤖 Auto-selecting shader: {} (based on audio analysis)", recommended_shader.name());
            self.set_shader(recommended_shader, context)?;
        }

        Ok(())
    }

    /// Get current performance quality level
    pub fn current_quality(&self) -> QualityLevel {
        self.performance_manager.current_quality()
    }

    /// Manually set performance quality level
    pub fn set_quality(&mut self, quality: QualityLevel) {
        self.performance_manager.set_quality(quality);
    }

    /// Get performance metrics report
    pub fn performance_report(&self) -> String {
        self.performance_manager.performance_report()
    }

    /// Get average FPS over recent history
    pub fn average_fps(&self) -> f32 {
        self.performance_manager.average_fps()
    }

    fn analyze_audio_for_shader(&self, audio: &AudioFeatures, rhythm: &RhythmFeatures) -> ShaderType {
        // Analyze audio characteristics to recommend optimal shader

        // High bass content -> Classic or Tunnel
        if audio.bass + audio.sub_bass > 0.7 {
            return if rhythm.tempo_confidence > 0.8 {
                ShaderType::Tunnel // Strong rhythm + bass = tunnel effect
            } else {
                ShaderType::Classic // Just bass = classic waves
            };
        }

        // High treble + onset activity -> Particle system
        if audio.treble + audio.presence > 0.6 && audio.onset_strength > 0.5 {
            return ShaderType::Particle;
        }

        // High pitch confidence + harmony -> Kaleidoscope
        if audio.pitch_confidence > 0.7 && rhythm.rhythm_stability > 0.6 {
            return ShaderType::Kaleidoscope;
        }

        // High spectral flux (dynamic changes) -> Parametric wave
        if audio.spectral_flux > 0.4 {
            return ShaderType::ParametricWave;
        }

        // High dynamic range -> Fractal
        if audio.dynamic_range > 0.6 {
            return ShaderType::Fractal;
        }

        // Default fallback
        ShaderType::Classic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_layout() {
        let desc = Vertex::desc();
        assert_eq!(desc.array_stride, std::mem::size_of::<Vertex>() as wgpu::BufferAddress);
        assert_eq!(desc.attributes.len(), 2);
    }

    #[test]
    fn test_geometry_data() {
        assert_eq!(VERTICES.len(), 4);
        assert_eq!(INDICES.len(), 6);

        // Verify quad covers full screen in NDC
        assert_eq!(VERTICES[0].position, [-1.0, -1.0, 0.0]); // Bottom-left
        assert_eq!(VERTICES[1].position, [1.0, -1.0, 0.0]);  // Bottom-right
        assert_eq!(VERTICES[2].position, [1.0, 1.0, 0.0]);   // Top-right
        assert_eq!(VERTICES[3].position, [-1.0, 1.0, 0.0]);  // Top-left
    }

    #[test]
    fn test_audio_analysis_for_shader() {
        // This would need to be implemented as an integration test
        // since it requires a full EnhancedFrameComposer instance
    }
}