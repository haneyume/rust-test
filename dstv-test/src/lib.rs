use dstv::prelude::*;
use std::fs;

pub fn convert_dstv(file_path: &str, output_path: &str) {
    let dstv = Dstv::from_file(file_path);
    assert_eq!(dstv.is_ok(), true);

    let mut dstv = dstv.unwrap();
    let svg = dstv.to_svg();

    fs::write(output_path, svg).expect("Unable to write file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        convert_dstv("./test/0008-SE0008.nc1", "./test/0008-SE0008.svg");

        convert_dstv("./test/A10-15.nc1", "./test/A10-15.svg");
        convert_dstv("./test/A10-17.nc1", "./test/A10-17.svg");
        convert_dstv("./test/A10-19.nc1", "./test/A10-19.svg");
        convert_dstv("./test/A10-32.nc1", "./test/A10-32.svg");
        convert_dstv("./test/C1-M1.nc1", "./test/C1-M1.svg");
        convert_dstv("./test/G1-M2.nc1", "./test/G1-M2.svg");
        convert_dstv("./test/G2-M4.nc1", "./test/G2-M4.svg");
    }

    #[test]
    fn convert_all_files() {
        let files = fs::read_dir("./test-nc1").unwrap();

        for file in files {
            let file = file.unwrap();
            let file_path = file.path();
            let file_path = file_path.to_str().unwrap();
            let output_path = file_path
                .replace("test-nc1", "test-svg")
                .replace(".nc1", ".svg");

            convert_dstv(file_path, &output_path);
        }
    }
}
