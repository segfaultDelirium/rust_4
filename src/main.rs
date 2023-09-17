use regex::Regex;



fn main() {
    println!("Hello, world!");

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

    regex();

    zadanie7();

    zadanie8();
    zadanie9();
}

fn zadanie9(){
    let list1 = get_sorted([5, 3, 7, 1, 4, 7, 1, 9, 1, 5, 2].to_vec());
    println!("list: {:?}", list1);

    let list2 = get_sorted([9,1,2,4,5,3,2].to_vec());
    println!("list2: {:?}", list2);

    let x = 9;

    


}



fn get_sorted(list: Vec<i32>) -> Vec<i32>{
    let mut list_copy = list.clone();
    list_copy.sort();
    return list_copy;
}

fn zadanie8(){
    println!("zadanie8");


    let sample_list = get_sorted([5, 3, 7, 1, 4, 7, 1, 9, 1, 5, 2].to_vec());
    println!("list: {:?}", sample_list);

    let indexof = binary_search(sample_list, 4);
    println!("index of 4 = {indexof}");

}

fn binary_search(list: Vec<i32>, searched_item: i32) -> usize{
    let middle_index = (list.len() as f64 / 2.0) as usize; 
    
    if list[middle_index] == searched_item{
        return middle_index;
    }
    if list[middle_index] < searched_item {
        return binary_search(list[0..middle_index].to_vec(), searched_item);
    }
    return binary_search(list[middle_index..list.len()].to_vec(), searched_item);
}

fn zadanie7(){

    let sample_list = [5, -3, 7, 2].to_vec();
    println!("list: {:?}", sample_list);

    let sum = sum_of_elements(&sample_list);
    println!("sum = {sum}");
    let product = product_of_elements(&sample_list);
    println!("product: {product}");

    let average = average_of_elements(&sample_list);
    println!("average: {average}");

    let average_of_positives = average_of_positive_elements(&sample_list);
    println!("average_of_positives: {average_of_positives}");

    let f5_result = f5(&sample_list);
    println!("f5_result = {f5_result}");
}

fn f5(list: &Vec<i32>) -> i32{
    list
        .into_iter()
        .enumerate()
        .fold(0, |acc, (k, _x)|{
            acc + list.into_iter().take(k).fold(1, |acc, x| acc * x)
        }
    )
}

fn average_of_positive_elements(list: &Vec<i32>) -> f64{
    let (count, sum) = list.into_iter()
    .fold((0, 0), 
        |acc, x| (acc.0 + 1, if *x > 0 {acc.1 + x} else {acc.1})
    );

    // println!("count: {count}, sum: {sum}");
    return sum as f64 / count as f64;
}

fn average_of_elements(list: &Vec<i32>) -> f64{
    let (count, sum) = list.into_iter()
        .fold((0, 0), 
            |acc, x| (acc.0 + 1, acc.1 + x)
        );

    // println!("count: {count}, sum: {sum}");
    return sum as f64 / count as f64;
}

fn product_of_elements(list: &Vec<i32>) -> i32{
    list.into_iter()
        .fold(1, |acc, x| acc * x)
}

fn sum_of_elements(list: &Vec<i32>) -> i32{
    list.into_iter()
        .fold(0, |acc, x| acc + x)
}


fn regex(){ 
    let re = Regex::new(r"rust").unwrap();

    let result = re.replace("I love rust!", "Rust");
    assert_eq!(result, "I love Rust!");
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

