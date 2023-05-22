use super::ffmpeg_builder::FFmpegBuilder;




impl FFmpegBuilder {


    pub fn set_speed(&mut self, speed: f64) -> &mut Self {
        self.video_filter("setpts", format!("PTS/{}", speed.to_string()));

        let mut clamped_audio_speed = speed;

        if speed != 1.0 {
        
            if clamped_audio_speed > 2.0 {
                clamped_audio_speed = 2.0;
            }else if clamped_audio_speed < 0.5 {
                clamped_audio_speed = 0.5;
            }
        }
        self.audio_filter("atempo", clamped_audio_speed.to_string());
        self
    }
}