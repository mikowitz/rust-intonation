//! Helpers for playback via [rodio](https://docs.rs/rodio).
use rodio::{
    source::{Amplify, SineWave, Source, TakeDuration},
    OutputStream, Sink,
};
use std::time::Duration;

/// Trait to allow playback using [rodio](https://docs.rs/rodio).
pub trait Play {
    fn play(&self);
}

pub(crate) fn play_interval(root_freq: f32, interval_freq: f32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let root = create_sine_wave(root_freq);
    let interval = create_sine_wave(interval_freq);

    sink.append(root);
    sink.append(interval);
    sink.sleep_until_end();
}

pub(crate) fn play_dyad(root_freq: f32, interval_freq: f32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let sink2 = Sink::try_new(&stream_handle).unwrap();

    let root = create_sine_wave(root_freq);
    let interval = create_sine_wave(interval_freq);

    sink.append(root);
    sink2.append(interval);
    sink.sleep_until_end();
    sink2.sleep_until_end();
}

pub(crate) fn create_sine_wave(freq: f32) -> Amplify<TakeDuration<SineWave>> {
    SineWave::new(freq)
        .take_duration(Duration::from_secs_f32(2.))
        .amplify(0.2)
}
