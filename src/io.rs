use crate::error::Result;
use crate::feedback::FeedbackItem;
use std::io::{Read, Write};

pub fn read_feedback<R: Read>(data_file: R) -> Result<Vec<FeedbackItem>> {
    serde_json::from_reader(data_file).map_err(Into::into)
}

pub fn save_feedback<W: Write>(data_file: W, feedback_data: &[FeedbackItem]) -> Result<()> {
    serde_json::to_writer(data_file, feedback_data).map_err(Into::into)
}
