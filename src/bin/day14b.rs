fn main() {
    let target = &[7, 6, 8, 0, 7, 1];
    let len = target.len();
    let mut recipes = vec![3, 7, 1, 0, 1, 0];
    let mut i: usize = 4;
    let mut j: usize = 3;

    let size: usize;
    loop {
        let sum = recipes[i] + recipes[j];
        if sum < 10 {
            recipes.push(sum);
            if recipes.ends_with(target) {
                size = recipes.len() - len;
                break;
            }
        } else {
            recipes.push(1);
            if recipes.ends_with(target) {
                size = recipes.len() - len;
                break;
            }
            recipes.push(sum - 10);
            if recipes.ends_with(target) {
                size = recipes.len() - len;
                break;
            }
        }
        i = (i + recipes[i] + 1) % recipes.len();
        j = (j + recipes[j] + 1) % recipes.len();
    }

    println!("{}", size);
}
