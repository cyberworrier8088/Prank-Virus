// This Code small complicated. Becuse This using GIF. 

use std::time::Duration;
use show_image::{ImageView, create_window}; 
use image::AnimationDecoder;

pub fn gif() -> Result<(), Box<dyn std::error::Error>> {// pub for public. pub keyword not use that not callble in main.rs can this file
    // 1. Open the GIF file by embedding it directly into the executable
    let gif_bytes = include_bytes!("giphy.gif"); // include_bytes bakes it into the .exe
    let reader = std::io::Cursor::new(gif_bytes); 
    
    // 2. Decode the GIF into its individual frames
    let decoder = image::codecs::gif::GifDecoder::new(reader)?;
    let frames = decoder.into_frames().collect_frames()?;

    // 5. Create a rendering window
    let window = create_window("GIF Player", Default::default())?;

    println!("Playing GIF. Close the window to exit.");

    // 4. Run an infinite loop to cycle through frames
    loop {
        for frame in &frames {
            // Convert the current frame's buffer into an image view
            let buffer = frame.buffer();
            let image_view = ImageView::new(// show_image function
                show_image::ImageInfo::rgba8(buffer.width(), buffer.height()),
                buffer.as_raw()
            );

            // Update the window with the new frame
            window.set_image("gif-frame", image_view)?;

            // Get the frame delay duration (default to 100ms if not specified)
            let delay: Duration = frame.delay().into();
            std::thread::sleep(delay);
        }
    }
}
