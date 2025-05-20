fn main() {
    let mut count = 0;
    'counting_up: loop{
        println!("count = {count}");
        let mut remain = 10;
        loop {
            println!("remaining = {remain}");
            if remain == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remain -= 1;
        }
        count += 1;
    }
    println!("end count = {count}");
    let a = [10,20,30,40,50,20,10];
    for elm in a {
        println!("the value is: {elm}");
    }

}
