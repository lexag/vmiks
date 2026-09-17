use crate::{
    error::MiksError,
    media::{frame::VideoFrame, source::VideoSource},
    playback::PlaybackState,
    project::model::Camera,
};
use egui::{Color32, TextureHandle, TextureOptions};
use gstreamer::{
    glib::object::ObjectExt,
    gobject::GObjectExtManualGst,
    prelude::{ElementExt, ElementExtManual, GstBinExtManual, PadExt},
    BufferRef, ClockTime, Element, ElementFactory, FlowSuccess, MessageType, Pipeline, Sample,
    SeekFlags, State,
};
use gstreamer_app::{AppSink, AppSinkCallbacks};
use std::{
    collections::VecDeque,
    path::Path,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

#[derive(Debug, Clone)]
pub struct VideoSample {
    pub sample: gstreamer::Sample,
    pub time: ClockTime,
}

impl VideoSample {}

#[derive(Debug)]
pub struct Decoder {
    pub pipeline: Pipeline,
    pub src: Element,
    pub sink: AppSink,
    pub sample_recv: Receiver<VideoSample>,
    pub unblock_gst_send: Sender<()>,
    pub latest_frame: VideoFrame,
    pub seeking: bool,
}

impl Decoder {
    const QUEUE_LIMIT: usize = 128;

    pub fn new(source: &Path) -> Result<Self, MiksError> {
        let pipeline = Pipeline::new();

        let uri = "https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm";

        // Create video source
        let src = ElementFactory::make("uridecodebin").build()?;
        src.set_property_from_str("uri", uri);

        let convert = ElementFactory::make("videoconvert").build()?;

        let snk = gstreamer_app::AppSink::builder()
            .caps(
                &gstreamer::Caps::builder("video/x-raw")
                    .field("format", "RGB")
                    .build(),
            )
            .max_buffers(1)
            .drop(true)
            .build();

        snk.set_property("sync", false);

        pipeline.add_many([&src, &convert, snk.as_ref()])?;

        convert.link(&snk)?;

        let convert_sink = convert.static_pad("sink").unwrap();

        src.connect_pad_added(move |_src, pad| {
            if pad.is_linked() {
                return;
            }

            let Some(caps) = pad.current_caps() else {
                return;
            };

            let Some(s) = caps.structure(0) else {
                return;
            };

            if !s.name().starts_with("video/") {
                return;
            }

            if let Err(err) = pad.link(&convert_sink) {
                eprintln!("Failed to link video: {err:?}");
            }
        });

        pipeline.set_state(gstreamer::State::Paused)?;

        // Create shared resources
        let latest_frame = VideoFrame {
            image: egui::ColorImage::filled([340, 200], Color32::BLUE),
            time: ClockTime::ZERO,
        };
        let latest_frame_c = latest_frame.clone();

        let (sample_send, sample_recv) = mpsc::sync_channel(Self::QUEUE_LIMIT);
        let (unblock_gst_send, unblock_gst_recv) = mpsc::channel();

        // Set sink callbacks
        snk.set_callbacks(
            AppSinkCallbacks::builder()
                .new_sample(move |appsink| {
                    let sample = match appsink.pull_sample() {
                        Ok(s) => s,
                        Err(_) => return Ok(FlowSuccess::Ok),
                    };
                    let time = appsink.query_position::<ClockTime>().unwrap_or_default();
                    let vsample = VideoSample { sample, time };

                    loop {
                        match sample_send.try_send(vsample.clone()) {
                            Ok(_) => break,
                            Err(_) => {
                                if unblock_gst_recv.try_recv().is_ok() {
                                    break;
                                }
                            }
                        }
                    }

                    Ok(FlowSuccess::Ok)
                })
                .build(),
        );

        println!("made pipeline");
        let bus = pipeline.bus().ok_or(MiksError::OptionNone)?;
        if let Some(msg) =
            bus.timed_pop_filtered(ClockTime::SECOND, &[MessageType::Error, MessageType::Eos])
        {
            println!("{msg:?}");
        }

        Ok(Self {
            src,
            sink: snk,
            sample_recv,
            unblock_gst_send,
            pipeline,
            latest_frame: latest_frame_c,
            seeking: false,
        })
    }

    fn unblock_gst(&self) {
        self.unblock_gst_send.send(());
    }

    fn extract_frame(&self, sample: VideoSample) -> VideoFrame {
        let buffer = sample.sample.buffer().unwrap();
        let caps = sample.sample.caps().unwrap();
        let info = gstreamer_video::VideoInfo::from_caps(caps).unwrap();
        let map = buffer.map_readable().unwrap();

        let stride = info.stride()[0] as usize;
        let width = info.width() as usize;
        let height = info.height() as usize;

        let row_size = width * 3;
        // RGB = 3 bytes/pixel

        let mut rgb = Vec::with_capacity(row_size * height);

        for y in 0..height {
            let offset = y * stride;
            rgb.extend_from_slice(&map.as_slice()[offset..offset + row_size]);
        }

        let image = egui::ColorImage::from_rgb([width, height], &rgb);

        let time = sample.time;

        VideoFrame { image, time }
    }
}

impl VideoSource for Decoder {
    fn duration(&self) -> f64 {
        todo!();
    }

    fn seek(&mut self, position: f64) -> Result<(), crate::error::MiksError> {
        self.unblock_gst();
        self.pipeline.seek_simple(
            SeekFlags::FLUSH | SeekFlags::KEY_UNIT,
            ClockTime::from_seconds_f64(position.max(0.0)),
        )?;
        while self.sample_recv.try_recv().is_ok() {}
        Ok(())
    }

    fn step(&mut self) -> Result<(), MiksError> {
        todo!()
    }

    fn play(&mut self) -> Result<(), crate::error::MiksError> {
        self.unblock_gst();
        self.pipeline.set_state(State::Playing);
        Ok(())
    }

    fn pause(&mut self) -> Result<(), crate::error::MiksError> {
        self.unblock_gst();
        self.pipeline.set_state(State::Paused);
        Ok(())
    }

    fn latest_frame(&self) -> &VideoFrame {
        &self.latest_frame
    }

    fn time(&self) -> ClockTime {
        self.latest_frame.time
    }

    fn playing(&self) -> bool {
        self.pipeline.current_state() == State::Playing
    }
}

#[derive(Default)]
pub struct DecoderSlot {
    decoder: Option<Decoder>,

    pub texture: Option<TextureHandle>,
}

impl DecoderSlot {
    pub fn new(texture: TextureHandle) -> Self {
        Self {
            decoder: None,
            texture: Some(texture),
        }
    }

    pub fn shutdown(&mut self) -> Result<(), MiksError> {
        if let Some(dec) = &self.decoder {
            dec.unblock_gst();
            dec.pipeline.set_state(gstreamer::State::Null)?;
        }
        Ok(())
    }

    pub fn populate(&mut self, source: &Path) -> Result<(), MiksError> {
        self.shutdown()?;

        self.decoder = Some(Decoder::new(source)?);

        Ok(())
    }

    pub fn update_texture(&mut self) {
        let Some(tex) = self.texture.as_mut() else {
            return;
        };
        let Some(decoder) = self.decoder.as_mut() else {
            return;
        };

        tex.set(
            decoder.latest_frame().image.clone(),
            TextureOptions::default(),
        );
    }

    pub fn keep_time(&mut self, playback: &PlaybackState, offset: f64) -> f64 {
        const RESYNC_THRES: u64 = 1_000_000_000;

        let Some(decoder) = self.decoder.as_mut() else {
            return 0.0;
        };

        if decoder.playing() != playback.playing() {
            if playback.playing() {
                decoder.play();
            } else {
                decoder.pause();
            }
        }

        let main_time = ClockTime::from_seconds_f64((playback.time() - offset).max(0.0));

        let mut frame_time = decoder.time();

        let mut latest_sample = None;
        while playback.playing() && frame_time < main_time {
            //|| decoder.seeking {
            let Ok(sample) = decoder.sample_recv.try_recv() else {
                //println!(
                //    "Channel ran out of images at frame_time {}., Stopping seek",
                //    frame_time
                //);
                decoder.seeking = false;
                break;
            };
            //println!("Grabbing image at frame_time {frame_time}.");
            frame_time = sample.time;
            latest_sample = Some(sample);
        }

        if let Some(sample) = latest_sample {
            decoder.latest_frame = decoder.extract_frame(sample);
        }

        let frame_time = decoder.time();
        let error_ns = main_time.abs_diff(frame_time.into());
        if error_ns > RESYNC_THRES {
            //println!(
            //    "frametime: {}, maintime: {} ==> error: {}ms, start seek.",
            //    frame_time,
            //    main_time,
            //    error_ns / 1000000
            //);
            if let Err(e) = decoder.seek(playback.time() - offset) {
                eprintln!("{e:?}");
            };

            if playback.playing() {
                let Ok(sample) = decoder.sample_recv.recv() else {
                    return 0.0;
                };
                decoder.latest_frame = decoder.extract_frame(sample);
            }
        }

        // hämta frames tills frame time inte ännu har hänt -> visa den framen.
        // om inte den just nu visade framen har hänt än -> hämta ingen ny frame
        // när just nu visade framen går över "nu" -> hämta ny frame (tills inkommande frame inte
        // har hänt än)

        frame_time.seconds_f64()
        //((error_ns / 1000) as f64 / 1000000.0) * if main_time > frame_time { -1.0 } else { 1.0 }
    }
}
