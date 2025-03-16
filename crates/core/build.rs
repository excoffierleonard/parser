fn main() {
    // Check for Tesseract
    match pkg_config::probe_library("tesseract") {
        Ok(_) => println!("Found Tesseract library!"),
        Err(_) => {
            println!("cargo:warning=Tesseract library not found in your system.");
            println!("cargo:warning=On Debian/Ubuntu, install with: apt install libtesseract-dev libleptonica-dev libclang-dev");
            println!("cargo:warning=On macOS, install with: brew install tesseract");
            println!("cargo:warning=On Windows, follow instructions at https://github.com/tesseract-ocr/tesseract");
        }
    }

    // Check for Leptonica (Tesseract dependency)
    match pkg_config::probe_library("lept") {
        Ok(_) => println!("Found Leptonica library!"),
        Err(_) => {
            if pkg_config::probe_library("tesseract").is_ok() {
                // If tesseract is installed but leptonica probe failed, it might be bundled
                println!("Assuming Leptonica is bundled with Tesseract");
            } else {
                println!("cargo:warning=Leptonica library not found in your system.");
                println!("cargo:warning=On Debian/Ubuntu, install with: apt install libleptonica-dev");
                println!("cargo:warning=On macOS, it should be installed with Tesseract");
            }
        }
    }
}