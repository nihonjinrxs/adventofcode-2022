use crate::parts::Parts;

pub fn run(part_number: Parts, input: &str) -> String {
    let mut elf_calories = collect_elf_calories(input).to_owned();
    let result = compute_result(part_number, &mut elf_calories);
    format!("{}", result)
}

fn compute_result(part_number: Parts, elf_calories: &mut Vec<i32>) -> i32 {
    match part_number {
        Parts::One => max_elf_calories(elf_calories, 1),
        Parts::Two => max_elf_calories(elf_calories, 3),
    }
}

fn max_elf_calories(elf_calories: &mut Vec<i32>, top_n: usize) -> i32 {
    elf_calories.sort_by(|a, b| b.cmp(a));
    elf_calories.truncate(top_n);
    let result = elf_calories.iter().sum();
    result
}

fn collect_elf_calories(input: &str) -> Vec<i32> {
    let mut elf_calories: Vec<i32> = vec![];
    let mut running_total: i32 = 0;
    for l in input.split('\n') {
        if l.is_empty() {
            elf_calories.push(running_total);
            running_total = 0;
        } else {
            running_total += l.parse::<i32>().unwrap();
        }
    }
    if running_total > 0 {
        elf_calories.push(running_total);
    }
    elf_calories
}

