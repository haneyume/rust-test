use dstv::prelude::*;
use std::fs;

pub fn convert_dstv(file_path: &str, output_path: &str) {
    let dstv = Dstv::from_file(file_path);
    assert_eq!(dstv.is_ok(), true);

    let mut dstv = dstv.unwrap();
    let svg = dstv.to_svg();

    fs::write(output_path, svg).expect("Unable to write file");
}

pub fn convert_all_files(input_dir: &str, output_dir: &str) {
    let dir = format!("./{}", input_dir);

    let files = fs::read_dir(dir).unwrap();

    for file in files {
        let file = file.unwrap();
        let file_path = file.path();
        let file_path = file_path.to_str().unwrap();
        let output_path = file_path
            .replace(input_dir, output_dir)
            .replace(".nc1", ".svg");

        convert_dstv(file_path, &output_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_file() {
        convert_dstv("./test-1-nc1/A10-15.nc1", "./test-1-svg/A10-15.svg");
    }

    #[test]
    fn test_all() {
        convert_all_files("test-1-nc1", "test-1-svg");
        convert_all_files("test-2-nc1", "test-2-svg");
        convert_all_files("test-3-nc1", "test-3-svg");
    }
}
