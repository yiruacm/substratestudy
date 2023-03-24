fn sum(muset: &[u32]) -> Option<u32> {
    let mut s: u32 =0;
    let mut overflow = false;
    for n in muset {
        match s.checked_add(*n) {
            None => {
                overflow = true;
                break;
            }
            Some(v) => {
                s = v;
            }
        }    
    }
    if overflow{
        return None;
    }
    Some(s)
}
fn main() {
    let muset1 = vec![2,4,6,8,10];
    let muset2 = vec![1,2,3,4,u32::MAX];
    println!("{:?}",sum(&muset1));
    println!("{:?}",sum(&muset2));
}
