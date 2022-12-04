pub fn vec_compare<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let match_count = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    match_count == a.len() && match_count == b.len()
}

pub struct TestCase<TInput, TExpected> {
    pub input: TInput,
    pub expected: TExpected,
}

impl<TInput: Clone, TExpected: Clone> TestCase<TInput, TExpected> {
    pub fn create_many(inputs: Vec<TInput>, expecteds: Vec<TExpected>) -> Vec<TestCase<TInput, TExpected>> {
        let pairs_iter = inputs.iter().zip(expecteds.iter());
        pairs_iter.map(|(input, expected)| TestCase{
            input: input.clone(),
            expected: expected.clone(),
        }).collect::<Vec<TestCase<TInput, TExpected>>>()
    }
}
