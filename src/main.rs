use clap::{Arg, App};
use std::process;
use std::fs;

pub mod kakasu;

fn main() {
    let matches = App::new("Kakusu")
        .version("1.0")
        .author("Renell Castro <renellc9819@gmail.com>")
        .about("Hide/Reveal secret messages in your images!")
        .arg(Arg::with_name("message")
            .short("m")
            .long("message")
            .value_name("MESSAGE")
            .help("The message to encode")
            .conflicts_with("file"))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("The text file to encode")
            .conflicts_with("message"))
        .arg(Arg::with_name("image")
            .short("i")
            .long("image")
            .value_name("IMAGE")
            .help("The image to store the message."))
        .arg(Arg::with_name("decode")
            .short("d")
            .takes_value(false)
            .help("Decodes the provided image.")
            .conflicts_with("file")
            .conflicts_with("message"))
        .get_matches();

    let (message, file) = (matches.value_of("message"), matches.value_of("file"));
    let image = match matches.value_of("image") {
        Some(img) => {
            match image::open(img) {
                Ok(img) => img,
                Err(e) => {
                    println!("{}", e);
                    process::exit(1);
                }
            }
        },
        _ => {
            println!("No image provided!");
            process::exit(1);
        }
    };

    if matches.is_present("decode") {
        match kakasu::decode_image(&image) {
            Ok(message) => {
                println!("Secret message was:\n{}", message);
            }
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }
    } else {

        let mut msg_bytes = match (message, file) {
            (Some(msg), None) => Vec::from(msg.as_bytes()),
            (None, Some(file)) => {
                if let Ok(bytes_vec) = fs::read(file) {
                    bytes_vec
                } else {
                    println!("File provided not valid!");
                    process::exit(1);
                }
            },
            (_, _) => {
                println!("No message or text file provided!");
                process::exit(1);
            }
        };

        match kakasu::encode_message(&mut msg_bytes, &image) {
            Ok(encoded_img) => {
                match encoded_img.save("secret.png") {
                    Err(e) => {
                        println!("Could not save encoded image: {}", e);
                        process::exit(1);
                    }
                    _ => { println!("Saved encoded image as secret.png"); }
                }
            },
            Err(e) => {
                println!("Could not encode message in image: {}", e);
                process::exit(1);
            }
        }
    }
}
