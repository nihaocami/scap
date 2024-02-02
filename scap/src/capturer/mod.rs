mod engine;

use std::sync::mpsc;

use crate::{
    device::display,
    frame::{Frame, FrameType},
};

#[derive(Debug, Clone, Copy, Default)]
pub enum Resolution {
    _480p,
    _720p,
    _1080p,
    _1440p,
    _2160p,
    _4320p,

    #[default]
    Captured,
}

impl Resolution {
    fn value(&self) -> [u32; 2] {
        match *self {
            Resolution::_480p => [640, 480],
            Resolution::_720p => [1280, 720],
            Resolution::_1080p => [1920, 1080],
            Resolution::_1440p => [2560, 1440],
            Resolution::_2160p => [3840, 2160],
            Resolution::_4320p => [7680, 4320],
            Resolution::Captured => {
                panic!(".value should not be called when Resolution type is Captured")
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CGPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Default, Clone)]
pub struct CGSize {
    pub width: f64,
    pub height: f64,
}
#[derive(Debug, Default, Clone)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}
#[derive(Debug, Default)]
pub struct Options {
    pub fps: u32,
    pub show_cursor: bool,
    pub show_highlight: bool,
    pub targets: Vec<display::Target>,

    // excluded targets will only work on macOS
    pub excluded_targets: Option<Vec<display::Target>>,
    pub output_type: FrameType,
    pub output_resolution: Resolution,
    pub source_rect: Option<CGRect>,
}

pub struct Capturer {
    engine: engine::Engine,
    rx: mpsc::Receiver<Frame>,
}

impl Capturer {
    pub fn new(options: Options) -> Capturer {
        let (tx, rx) = mpsc::channel::<Frame>();
        let engine = engine::Engine::new(&options, tx);

        Capturer { engine, rx }
    }

    // TODO
    // Prevent starting capture if already started
    pub fn start_capture(&mut self) {
        self.engine.start();
    }
    pub fn stop_capture(&mut self) {
        self.engine.stop();
    }

    pub fn get_next_frame(&self) -> Result<Frame, mpsc::RecvError> {
        self.rx.recv()
    }
}
