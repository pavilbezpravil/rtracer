use std::time::Instant;

pub struct FrameCounter {
    last_fps_update: Option<Instant>,
    total_frame: usize,
    fps: f32,
    cur_fps_count: usize,
}

impl FrameCounter {
    pub fn new() -> FrameCounter {
        FrameCounter { last_fps_update: None, total_frame: 0, fps: 0., cur_fps_count: 0 }
    }

    pub fn next_frame(&mut self) -> Option<f32> {
        if self.last_fps_update == None {
            self.last_fps_update = Some(Instant::now());
        }

        self.total_frame += 1;
        self.cur_fps_count += 1;

        let elapsed = (self.last_fps_update.unwrap().elapsed().as_millis() / 1000) as f32;
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
}