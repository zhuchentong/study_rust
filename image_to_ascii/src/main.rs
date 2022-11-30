use image::GenericImageView;

/**
 *  获取图片像素值
 */
fn fetch_image_bytes(dirname: &str, scale: u32) -> Vec<Vec<u8>> {
    let img = image::open(dirname).expect("Not Find Image");

    let (width, height) = img.dimensions();
    println!("{:?}", (width, height));
    let mut data: Vec<Vec<u8>> = Vec::new();

    for y in 0..height {
        if y % (scale * 2) == 0 {
            let mut row = Vec::new();
            for x in 0..width {
                if x % scale == 0 {
                    let pixel = img.get_pixel(x, y);
                    let pixel = if pixel[3] == 0 {
                        0
                    } else {
                        pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3
                    };
                    row.push(pixel);
                }
            }

            data.push(row);
        }
    }

    data
}

/**
 * 图片像素转ASCII码
 */
fn bytes_to_ascii(bytes: Vec<Vec<u8>>) -> Vec<Vec<char>> {
    let ascii = [' ', '.', ',', '-', '~', '+', '=', '@'];

    bytes
        .iter()
        .map(|row| {
            row.iter()
                .map(|pixel| ascii[(pixel / 32) as usize])
                .collect()
        })
        .collect()
}

/**
 * 打印ASCII码
 */
fn print_ascii(asciis: Vec<Vec<char>>) {
    for row in asciis {
        for ascii in row {
            print!("{}", ascii)
        }
        println!("")
    }
}
fn main() {
    let bytes = fetch_image_bytes("image.png", 10);
    let asciis = bytes_to_ascii(bytes);

    print_ascii(asciis);
}
