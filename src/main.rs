use anyhow::Result;
use gstreamer::glib::WeakRef;
use image::{ColorType, DynamicImage, ImageFormat};
extern crate gstreamer as gst;
extern crate gstreamer_app as gst_app;
extern crate gstreamer_video as gst_video;
use gst::element_error;
use gst::glib;
use gst::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    gst::init()?;
    
    let pipeline = gst::parse_launch(&format!(
        "multifilesrc location=src/example%01d.jp2 is-live=true pattern=ball ! image/jp2,framerate=1/1 ! avdec_jpeg2000 ! videoconvert ! queue ! x264enc ! queue  ! mp4mux ! filesink location=image.mp4"
    ))?
    .downcast::<gst::Pipeline>()
    .expect("Expected a gst::Pipeline");

    pipeline.set_state(gst::State::Playing)?;

    Ok(())
}
