pub fn task1() -> i32 {
    let data: Vec<Vec<i32>> = include_str!("input.txt")
        .lines()
        .map(|x| x.split("x").map(|c| c.parse::<i32>().unwrap()).collect())
        .collect();

    data.iter().map(|x| calc_dimensions(x)).sum()
}

fn calc_dimensions(dimensions: &Vec<i32>) -> i32 {
    let mut result = 0;
    let smallest = dimensions.iter().max().unwrap();
    let mut smallest = smallest * smallest;
    for i in 0..dimensions.len() - 1 {
        for j in i + 1..dimensions.len() {
            let cur_dimensions = dimensions[i] * dimensions[j];
            if smallest > cur_dimensions {
                smallest = cur_dimensions
            }
            result += cur_dimensions
        }
    }
    return 2 * result + smallest;
}
