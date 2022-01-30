use indicatif::{ProgressBar, ProgressStyle};

#[repr(transparent)]
pub struct UwUProgressBar(ProgressBar);

impl UwUProgressBar {
    #[inline]
    pub fn new() -> UwUProgressBar {
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.set_style(
            ProgressStyle::default_spinner().template("{spinner:.green} [{elapsed_precise}] {msg}"),
        );
        progress_bar.enable_steady_tick(100);

        UwUProgressBar(progress_bar)
    }

    #[inline]
    pub fn finish(&self, message: &'static str) {
        self.0.finish_with_message(message);
    }
}
