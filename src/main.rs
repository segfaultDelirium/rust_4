

fn main() {
    println!("Hello, world!");

    zadanie1();
}

fn zadanie1(){
    println!("zadanie1");

    // let sample_list: Vec<i32> = Vec::new();
    let sample_list = [5, 3, 7, 1, 4, 7, 1, 9, 1, 5, 2].to_vec();
    println!("list: {:?}", sample_list);

    let min_value = find_min(&sample_list);
    println!("min value in a list is: {min_value}");

    let min_value_index = find_first_index_of_min(&sample_list);
    println!("first index of min value: {min_value_index}");
    let last_index_of_min  =find_last_index_of_min(&sample_list);
    println!("last index of min value: {last_index_of_min}");

    let (min, max) = find_min_and_max(&sample_list);
    println!("min: {min}, max: {max}");
}

fn find_min_and_max(list: &Vec<i32>) -> (i32, i32){
    list.into_iter()
        .fold((i32::MAX,  i32::MIN), 
            |acc, x: &i32| {
                let new_min = if *x < acc.0 {*x} else {acc.0};
                let new_max = if *x > acc.1 {*x} else {acc.1};
                return (new_min, new_max);
            }
        )
}


fn find_min(list: &Vec<i32>) -> i32{
    list.into_iter()
        .fold(i32::MAX, 
            |acc, x| if x < &acc {*x} else {acc}
        )
}

fn find_first_index_of_min(list: &Vec<i32>) -> usize{
    list.into_iter()
        .enumerate()
        .fold((0, i32::MAX), 
            |acc, (i, x)| 
            if x < &acc.1 {(i, *x)} else {acc}
    ).0
}

fn find_last_index_of_min(list: &Vec<i32>) -> usize{
    list.into_iter()
        .enumerate()
        .fold((0, i32::MAX), 
            |acc, (i, x)| 
            if x <= &acc.1 {(i, *x)} else {acc}
    ).0
}

