use std::cmp::max;

pub fn run(input: &str) -> String {
    let mut elf_calories = 0;
    let mut max_elf_calories = 0;
    for l in input.split('\n') {
        if l.is_empty() {
            max_elf_calories = max(elf_calories, max_elf_calories);
            elf_calories = 0;
        } else {
            elf_calories += l.parse::<i32>().unwrap();
        }
    }
    max_elf_calories = max(elf_calories, max_elf_calories);
    format!("{}", max_elf_calories)
}
