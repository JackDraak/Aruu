use cpal::{Device, Stream, SampleFormat, StreamConfig, traits::*};
use rodio::{Decoder, OutputStream, Sink};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use anyhow::{Result, anyhow};

use super::{FftAnalyzer, AudioFeatures, AdvancedAudioAnalyzer};

const BUFFER_SIZE: usize = 1024;
const SAMPLE_RATE: u32 = 44100;

pub struct AudioProcessor {
    _stream: Option<Stream>,
    _output_stream: Option<OutputStream>,
    sink: Option<Sink>,
    audio_buffer: Arc<Mutex<VecDeque<f32>>>,
    fft_analyzer: FftAnalyzer,
    advanced_analyzer: AdvancedAudioAnalyzer,
    sample_rate: f32,
}

impl AudioProcessor {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow!("No input device available"))?;

        let config = device.default_input_config()?;
        let sample_rate = config.sample_rate().0 as f32;

        let audio_buffer = Arc::new(Mutex::new(VecDeque::with_capacity(BUFFER_SIZE * 4)));
        let buffer_clone = Arc::clone(&audio_buffer);

        let stream = Self::build_input_stream(&device, config, buffer_clone)?;

        let (_output_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        Ok(Self {
            _stream: Some(stream),
            _output_stream: Some(_output_stream),
            sink: Some(sink),
            audio_buffer,
            fft_analyzer: FftAnalyzer::new(BUFFER_SIZE),
            advanced_analyzer: AdvancedAudioAnalyzer::new(sample_rate),
            sample_rate,
        })
    }

    pub fn new_default() -> Self {
        Self {
            _stream: None,
            _output_stream: None,
            sink: None,
            audio_buffer: Arc::new(Mutex::new(VecDeque::new())),
            fft_analyzer: FftAnalyzer::new(BUFFER_SIZE),
            advanced_analyzer: AdvancedAudioAnalyzer::new(SAMPLE_RATE as f32),
            sample_rate: SAMPLE_RATE as f32,
        }
    }

    fn build_input_stream(
        device: &Device,
        config: cpal::SupportedStreamConfig,
        audio_buffer: Arc<Mutex<VecDeque<f32>>>,
    ) -> Result<Stream> {
        let sample_format = config.sample_format();
        let config: StreamConfig = config.into();

        let stream = match sample_format {
            SampleFormat::F32 => device.build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    Self::write_input_data(data, &audio_buffer);
                },
                |err| eprintln!("Error in audio stream: {}", err),
                None,
            )?,
            SampleFormat::I16 => device.build_input_stream(
                &config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    let float_data: Vec<f32> = data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                    Self::write_input_data(&float_data, &audio_buffer);
                },
                |err| eprintln!("Error in audio stream: {}", err),
                None,
            )?,
            SampleFormat::U16 => device.build_input_stream(
                &config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    let float_data: Vec<f32> = data.iter()
                        .map(|&s| (s as f32 - u16::MAX as f32 / 2.0) / (u16::MAX as f32 / 2.0))
                        .collect();
                    Self::write_input_data(&float_data, &audio_buffer);
                },
                |err| eprintln!("Error in audio stream: {}", err),
                None,
            )?,
            _ => return Err(anyhow!("Unsupported sample format: {:?}", sample_format)),
        };

        stream.play()?;
        Ok(stream)
    }

    fn write_input_data(input: &[f32], buffer: &Arc<Mutex<VecDeque<f32>>>) {
        if let Ok(mut buffer) = buffer.lock() {
            for &sample in input {
                if buffer.len() >= BUFFER_SIZE * 4 {
                    buffer.pop_front();
                }
                buffer.push_back(sample);
            }
        }
    }

    pub fn process_frame(&mut self) -> Result<AudioFeatures> {
        let samples = self.get_audio_samples();

        if samples.len() < BUFFER_SIZE {
            return Ok(AudioFeatures::new());
        }

        let frequency_bins = self.fft_analyzer.process_audio(&samples);

        // Use advanced analyzer for full temporal analysis including spectral flux and dynamic range
        let time_domain_samples = if samples.len() >= BUFFER_SIZE {
            Some(&samples[..BUFFER_SIZE])
        } else {
            None
        };

        let features = self.advanced_analyzer.analyze_with_context(
            frequency_bins,
            time_domain_samples
        );

        Ok(features)
    }

    fn get_audio_samples(&self) -> Vec<f32> {
        if let Ok(buffer) = self.audio_buffer.lock() {
            buffer.iter().copied().collect()
        } else {
            Vec::new()
        }
    }

    pub fn play_from_file(&mut self, file_path: &str) -> Result<()> {
        if let Some(ref sink) = self.sink {
            let file = std::fs::File::open(file_path)?;
            let decoder = Decoder::new(file)?;
            sink.append(decoder);
            Ok(())
        } else {
            Err(anyhow!("No audio output available"))
        }
    }

    pub fn is_playing(&self) -> bool {
        self.sink.as_ref().map_or(false, |sink| !sink.empty())
    }

    pub fn stop(&self) {
        if let Some(ref sink) = self.sink {
            sink.stop();
        }
    }

    pub fn pause(&self) {
        if let Some(ref sink) = self.sink {
            sink.pause();
        }
    }

    pub fn resume(&self) {
        if let Some(ref sink) = self.sink {
            sink.play();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_processor_creation() {
        let processor = AudioProcessor::new_default();
        assert_eq!(processor.sample_rate, SAMPLE_RATE as f32);
    }

    #[test]
    fn test_process_frame_empty() {
        let mut processor = AudioProcessor::new_default();
        let result = processor.process_frame();
        assert!(result.is_ok());
        let features = result.unwrap();
        assert_eq!(features.bass, 0.0);
        assert_eq!(features.overall_volume, 0.0);
    }
}