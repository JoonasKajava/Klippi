use crate::modules::utils::math::clamp;

use super::ffmpeg_builder::FFmpegBuilder;

impl FFmpegBuilder {
    pub fn set_speed(&mut self, speed: f64) -> &mut Self {
        self.video_filter("setpts", format!("PTS/{}", speed));

        let clamped_audio_speed = clamp(speed, 0.5, 2.0);

        self.audio_filter("atempo", clamped_audio_speed.to_string());
        self
    }
}
