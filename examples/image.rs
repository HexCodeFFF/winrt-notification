extern crate winrt_notification;
use std::path::Path;
use winrt_notification::{
    IconCrop,
    Toast,
};

fn main() {
    Toast::new("application that needs a toast with an image")
        .hero(&format!("file://{}", Path::new("C:\\absolute\\path\\to\\image.jpeg").display()), "alt text")
        .icon(
            &format!("file://{}", Path::new("c:/this/style/works/too/image.png").display()),
            IconCrop::Circular,
            "alt text",
        )
        .title("Lots of pictures here")
        .text1("One above the text as the hero")
        .text2("One to the left as an icon, and several below")
        .image(&format!("file://{}", Path::new("c:/photos/sun.png").display()), "the sun")
        .image(&format!("file://{}", Path::new("c:/photos/moon.png").display()), "the moon")
        .sound(None) // will be silent
        .show()
        .expect("notification failed");
}
