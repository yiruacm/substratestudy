fn bubble_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {
    for _i in 0..list.len() {
        for j in 0..list.len() - 1 {
            
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
    list
}

fn main() {
 
    let mut list = vec![99, 39, 3, 10, 47, 88, 34, 250, 99];
    bubble_sort(&mut list);
    println!("{:?}  ", list);
 
    let mut list = vec!['Z', 'k', 'U', 'B', 'a', 'A', 'p', 'l', 'J'];
    bubble_sort(&mut list);
    println!("{:?}  ", list);
}