pub fn task1() -> i32 {
    let data = parse_data();
    data.iter().map(|x| calc_dimensions(x)).sum()
}

pub fn task2() -> i32 {
    let data = parse_data();
    data.iter().map(|x| calc_ribbon(x)).sum()
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

fn calc_ribbon(dimensions: &Vec<i32>) -> i32 {
    let mut dims = dimensions.clone();
    dims.sort();
    return dims[0] * 2 + dims[1] * 2 + dims[0] * dims[1] * dims[2];
}

fn parse_data() -> Vec<Vec<i32>> {
    include_str!("input.txt")
        .lines()
        .map(|x| x.split("x").map(|c| c.parse::<i32>().unwrap()).collect())
        .collect()
}
