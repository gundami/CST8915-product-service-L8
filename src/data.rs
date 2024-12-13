use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Apple - AirPods Pro (2nd Generation)".to_string(),
            price: 249.99,
            description: "Wireless earbuds with active noise cancellation, transparency mode, and adaptive sound technology for immersive listening.".to_string(),
            image: "/airpods.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Atari 2600+".to_string(),
            price: 299.99,
            description: "The Atari 2600+ is a modernized version of the classic Atari 2600, featuring HDMI output, high cartridge compatibility, and a retro design for an authentic gaming experience.".to_string(),
            image: "/atari.jpg".to_string()
        },
        Product {
            id: 3,
            name: "IBM 5100".to_string(),
            price: 499.99,
            description: "The IBM 5100, released in 1975, was one of the first portable computers, featuring a built-in monitor, keyboard, and the ability to run APL and BASIC programming languages.".to_string(),
            image: "/ibm5100.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Insignia™ - 55 Class F30 Series LED 4K UHD Smart Fire TV".to_string(),
            price: 399.99,
            description: "A 4K UHD TV with integrated Fire TV functionality, offering direct access to streaming platforms and a voice assistant for hands-free control.".to_string(),
            image: "/insignia.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Apple - 10.9-Inch iPad (10th Generation)".to_string(),
            price: 329.99,
            description: "A versatile tablet with 64GB storage and Wi-Fi connectivity, suitable for multimedia, productivity, and creative tasks.".to_string(),
            image: "/ipad.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Apple - 13-inch MacBook Air with M2 Chip".to_string(),
            price: 999.99,
            description: "Features a sleek design, 16GB memory, 256GB SSD, and the Apple M2 chip for enhanced performance. Perfect for productivity and portability.".to_string(),
            image: "/macbook.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Boost Mobile - Moto G Play (2024)".to_string(),
            price: 99.99,
            description: "A prepaid smartphone with a 90Hz display, 50MP camera, and a long-lasting 5000mAh battery.".to_string(),
            image: "/moto.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Polaroid Spectra".to_string(),
            price: 79.99,
            description: "The Polaroid Spectra is an instant film camera series known for its wider-format photos, advanced features like autofocus and multiple exposure modes, and its sleek, modern design.".to_string(),
            image: "/polaroid.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Samsung - Galaxy A35 5G Clear Case".to_string(),
            price: 19.99,
            description: "A transparent case designed for the Galaxy A35, offering protection while showcasing the phone's original color.".to_string(),
            image: "/samsung.jpg".to_string()
        },
        Product {
            id: 10,
            name: "TP-Link - Deco X50 Outdoor Wi-Fi 6 Mesh Router".to_string(),
            price: 299.99,
            description: "Extends ultra-performance mesh Wi-Fi outdoors, providing up to 2,500 sq. ft. of coverage with waterproof and dustproof design.".to_string(),
            image: "/tplink.jpg".to_string()
        }
    ]
}