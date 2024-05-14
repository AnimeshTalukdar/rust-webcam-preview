use eye_hal::format::PixelFormat;
use eye_hal::traits::{Context, Device, Stream};
use eye_hal::PlatformContext;
// use image::DynamicImage;
use show_image::{create_window, ImageInfo, ImageView};
#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = show_image::create_window("Image", Default::default())?;
    // Create a context
    let ctx = PlatformContext::default();

    // Query for available devices.
    let devices = ctx.devices()?;
    // print devices
    for device in &devices {
        println!("Device: {:?}", device);
    }
    // First, we need a capture device to read images from. For this example, let's just choose
    // whatever device is first in the list.
    let dev = ctx.open_device(&devices[0].uri)?;

    println!("");
    // Query for available streams and just choose the first one.
    let streams = dev.streams()?;
    let mut streamidx = 0;
    // print all streams with pixfmt and index of stream
    for (index, stream) in streams.iter().enumerate() {
        if stream.pixfmt == PixelFormat::Rgb(24) {
            streamidx = index;
            break;
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

    // wait
    // std::thread::sleep(std::time::Duration::from_secs(10));
    loop {
        let frame = stream
            .next()
            .expect("Stream is dead")
            .expect("Failed to capture frame");
        let frame = match stream_desc.pixfmt {
            PixelFormat::Rgb(24) => frame.to_vec(), // Convert to Vec<u8>
            _ => unimplemented!(),
        };

        // let image_buffer = image::ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(
        //     stream_desc.width,
        //     stream_desc.height,
        //     frame.clone(), // Use cloned frame
        // )
        // .ok_or("failed to convert bytes to an image")?;
        // Display the image
        // let dynamic_image = DynamicImage::ImageRgb8(image_buffer);
        // let image_info = ImageInfo::rgb8(stream_desc.width, stream_desc.height);
        // let image_view = ImageView::new(
        //     ImageInfo::rgb8(stream_desc.width, stream_desc.height),
        //     frame.as_slice(),
        // );
        let imageview = ImageView::new(
            ImageInfo::rgb8(stream_desc.width, stream_desc.height),
            &frame,
        );
        window.set_image("Image", imageview)?;

        // Break after capturing one picture
        if var == 10000 {
            break Ok(());
        } else {
            var += 1;
        }
    }
}

// use show_image::{create_window, ImageInfo, ImageView};

// #[show_image::main]
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // printcurrent folder location
//     println!("{:?}", std::env::current_dir());
//     //  file = ../image.png read the image into an array
//     let mut imagearray: Vec<u8> = vec![0; 1920 * 1080 * 3];
//     let image = image::open("image.png")?;
//     println!("{:?}", image);
//     let image = image.to_rgb8();
//     for (i, pixel) in image.pixels().enumerate() {
//         imagearray[i * 3] = pixel[0];
//         imagearray[i * 3 + 1] = pixel[1];
//         imagearray[i * 3 + 2] = pixel[2];
//     }
//     // let mut arr: [u8; 1920 * 1080 * 3] = [0; 1920 * 1080 * 3];
//     // for i in 0..1920 * 1080 * 3 {
//     //     arr[i] = imagearray[i];
//     // }

//     let image = ImageView::new(ImageInfo::rgb8(1920, 1080), &imagearray);

//     // Create a window with default options and display the image.
//     let window = create_window("image", Default::default())?;
//     window.set_image("image-001", image)?;
//     // wait
//     std::thread::sleep(std::time::Duration::from_secs(10));

//     Ok(())
// }
