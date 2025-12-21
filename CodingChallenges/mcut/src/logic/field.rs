use crate::file::File;

pub fn process(input_file: &File, index: std::option::Option<usize>) {
    let actual_index = index.expect("No field index provided!");
    let lines = input_file.contents.split("\n").collect::<Vec<&str>>();

    for line in lines {
        let delimeter = "\t";
        let item = line.split(delimeter).collect::<Vec<&str>>()[actual_index];
        println!("{item}");
    }
}
