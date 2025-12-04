use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(size: u64, operation_desc: &str) -> ProgressBar {
  // create progress bar and set bar max (length) to len of file
  let progress_bar: ProgressBar = ProgressBar::new(size);
  // style the progress bar
  progress_bar.set_style(ProgressStyle::with_template(
    // The Template:
    // {spinner}     = A little spinning animation
    // {bar:40}      = The bar itself, 40 characters wide
    // {bytes}       = Current progress (automatically converts to MB/GB)
    // {total_bytes} = Total size (also auto-converted)
    // {eta}         = Estimated Time Remaining
    "{msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
  ).unwrap());

  // set message
  progress_bar.set_message(operation_desc.to_string());

  progress_bar
}