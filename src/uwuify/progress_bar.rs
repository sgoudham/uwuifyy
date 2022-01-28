use indicatif::{ProgressBar, ProgressStyle};

pub struct UwUProgressBar {
    downloaded_bytes: u64,
    progress_bar: ProgressBar,
}

impl UwUProgressBar {
    pub fn new(file_total_bytes: u64) -> UwUProgressBar {
        let progress_bar = ProgressBar::new(file_total_bytes);
        progress_bar.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:60.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
            .progress_chars("#>-"));

        UwUProgressBar {
            downloaded_bytes: 0,
            progress_bar,
        }
    }

    pub fn update_progess(&mut self, bytes_read_in: usize) {
        self.downloaded_bytes += bytes_read_in as u64;
        self.progress_bar.set_position(self.downloaded_bytes);
    }

    pub fn finish(&self, message: &'static str) {
        self.progress_bar.finish_with_message(message);
    }
}