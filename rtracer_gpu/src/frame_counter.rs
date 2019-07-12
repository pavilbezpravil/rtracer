use std::time::Instant;

pub struct FrameCounter {
    last_fps_update: Option<Instant>,
    last_frame: Option<Instant>,
    time_last_frame: f32,
    total_frame: usize,
    fps: f32,
    cur_fps_count: usize,
}

impl FrameCounter {
    pub fn new() -> FrameCounter {
        FrameCounter { last_fps_update: None, last_frame: None, time_last_frame: 0., total_frame: 0, fps: 0., cur_fps_count: 0 }
    }

    /// return fps of last sec
    pub fn next_frame(&mut self) -> Option<f32> {
        if self.last_fps_update == None {
            let now = Instant::now();

            self.last_fps_update = Some(now);
            self.last_frame = Some(now);
        }

        if let Some(last_frame) = self.last_frame {
            let elapsed = (last_frame.elapsed().as_micros() as f64 / 1_000_000.) as f32;
            self.time_last_frame = elapsed;
        }
        self.last_frame = Some(Instant::now());


        self.total_frame += 1;
        self.cur_fps_count += 1;

        let elapsed = (self.last_fps_update.unwrap().elapsed().as_micros() as f64 / 1_000_000.) as f32;
        if elapsed > 1. {
            self.fps = self.cur_fps_count as f32 / elapsed;

            self.last_fps_update = Some(Instant::now());
            self.cur_fps_count = 0;
            Some(self.fps)
        } else {
            None
        }
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }

    pub fn total_frame(&self) -> usize {
        self.total_frame
    }

    pub fn time_last_frame(&self) -> f32 {
        self.time_last_frame
    }
}