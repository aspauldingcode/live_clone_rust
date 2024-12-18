use rodio::{OutputStream, Sink, Source};

pub struct TimeSignature {
    pub numerator: u8,
    pub denominator: u8,
}

impl TimeSignature {
    fn beat_multiplier(&self) -> f64 {
        match self.denominator {
            2 => 2.0,    // Half notes
            4 => 1.0,    // Quarter notes (standard)
            8 => 0.5,    // Eighth notes
            16 => 0.25,  // Sixteenth notes
            _ => 1.0,    // Default to quarter notes
        }
    }
}

pub struct Metronome {
    sink: Sink,
    _stream: OutputStream,
    bpm: f64,
    time_signature: TimeSignature,
}

impl Metronome {
    pub fn new(bpm: f64) -> Result<Self, String> {
        let (_stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to open audio output: {}", e))?;
            
        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| format!("Failed to create audio sink: {}", e))?;

        Ok(Self {
            sink,
            _stream,
            bpm,
            time_signature: TimeSignature { numerator: 4, denominator: 4 },
        })
    }

    fn generate_click(sample_rate: u32, frequency: f32, volume: f32) -> Vec<f32> {
        let duration = 0.01; // 10ms click
        let samples = (sample_rate as f32 * duration) as usize;
        
        (0..samples)
            .map(|i| {
                let t = i as f32 / sample_rate as f32;
                if t < duration {
                    let envelope = (-t * 100.0).exp();
                    let signal = (t * frequency * 2.0 * std::f32::consts::PI).sin();
                    signal * envelope * volume
                } else {
                    0.0
                }
            })
            .collect()
    }

    pub fn start(&mut self) {
        let sample_rate = 44100;
        // Adjust beat duration based on time signature denominator
        let beat_duration = 60.0 / self.bpm * self.time_signature.beat_multiplier();
        let samples_per_beat = (sample_rate as f64 * beat_duration) as usize;
        
        // Generate clicks for different beat positions
        let accent_click = Self::generate_click(sample_rate, 880.0, 0.3);  // First beat
        let normal_click = Self::generate_click(sample_rate, 440.0, 0.2);  // Other beats
        
        // Create a bar based on time signature
        let mut bar = Vec::new();
        for beat in 0..self.time_signature.numerator {
            let click = if beat == 0 { &accent_click } else { &normal_click };
            bar.extend(vec![0.0; samples_per_beat - click.len()]);
            bar.extend(click);
        }

        let source = rodio::buffer::SamplesBuffer::new(1, sample_rate, bar)
            .repeat_infinite();
            
        self.sink.append(source);
        self.sink.play();
    }

    pub fn stop(&mut self) {
        self.sink.stop();
    }

    pub fn set_bpm(&mut self, bpm: f64) {
        self.bpm = bpm;
        if !self.sink.empty() {
            self.stop();
            self.start();
        }
    }

    pub fn set_time_signature(&mut self, numerator: u8, denominator: u8) {
        self.time_signature = TimeSignature { numerator, denominator };
        if !self.sink.empty() {
            self.stop();
            self.start();
        }
    }
} 