use crate::file::File;

pub fn process(input_file: &File, index: std::option::Option<usize>) {
    let actual_index = index.expect("No field index provided!");
    let lines = input_file.contents.split("\n").collect::<Vec<&str>>();

    for line in lines {
        let delimeter = "\t";
        let item = match line.split(delimeter).collect::<Vec<&str>>().get(actual_index) {
            Some(i) => i,
            None => ""
        };
        println!("{item}");
    }
}
