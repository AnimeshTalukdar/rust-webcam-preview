use eye_hal::format::PixelFormat;
use eye_hal::traits::{Context, Device, Stream};
use eye_hal::PlatformContext;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a context
    let ctx = PlatformContext::default();

    // Query for available devices.
    let devices = ctx.devices()?;
    // printdevices
    for device in &devices {
        println!("Device: {:?}", device);
    }
    // First, we need a capture device to read images from. For this example, let's just choose
    // whatever device is first in the list.
    //
    let dev = ctx.open_device(&devices[2].uri)?;

    println!("");
    // Query for available streams and just choose the first one.
    let streams = dev.streams()?;
    let mut streamidx = 0;
    // print all stream with pixfmt and index of stream
    for (index, stream) in streams.iter().enumerate() {
        if stream.pixfmt == PixelFormat::Rgb(24) {
            streamidx = index;
        }
        println!("Stream {}: {:?}", index, stream.pixfmt);
    }
    println!("");
    println!("{:?}", streamidx);
    let stream_desc = streams[streamidx].clone();
    println!("Stream: {:?}", stream_desc);

    // Since we want to capture images, we need to access the native image stream of the device.
    // The backend will internally select a suitable implementation for the platform stream. On
    // Linux for example, most devices support memory-mapped buffers.
    let mut stream = dev.start_stream(&stream_desc)?;

    // Here we create a loop and just capture images as long as the device produces them. Normally,
    // this loop will run forever unless we unplug the camera or exit the program.
    println!("Imageformat: {:?}", stream_desc.pixfmt);
    let mut var = 0;
    loop {
        let frame = stream
            .next()
            .expect("Stream is dead")
            .expect("Failed to capture frame");
        let frame = match stream_desc.pixfmt {
            PixelFormat::Rgb(24) => frame.to_vec(), // No conversion needed
            _ => unimplemented!(),
        };

        let image_buffer = image::ImageBuffer::<image::Rgb<u8>, &[u8]>::from_raw(
            stream_desc.width,
            stream_desc.height,
            &frame,
        )
        .ok_or("failed to convert bytes to an image")?;

        // Save the image
        image_buffer.save("image.png")?;

        // Break after capturing one picture
        if var == 0 {
            break Ok(());
        } else {
            var = var + 1;
        }
    }
}
