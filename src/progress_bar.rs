use indicatif::{ProgressBar, ProgressStyle};

#[repr(transparent)]
pub struct UwUProgressBar(ProgressBar);

impl UwUProgressBar {
    #[inline]
    pub fn new(file_total_bytes: u64) -> UwUProgressBar {
        let progress_bar = ProgressBar::new(file_total_bytes);
        progress_bar.set_style(ProgressStyle::default_bar()
                                   .template("{spinner:.green} [{elapsed_precise}] [{bar:60.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
                                   .progress_chars("#>-"));

        UwUProgressBar(progress_bar)
    }

    #[inline]
    pub fn update_progess(&mut self, bytes_read_in: usize) {
        self.0.inc(bytes_read_in as u64);
    }

    #[inline]
    pub fn finish(&self, message: &'static str) {
        self.0.finish_with_message(message);
    }
}
