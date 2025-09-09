fn main() {
    println!("Hello, world!");

    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");

    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");

    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;

    for &num in numbers.iter() {
        if num > max {
            max = num;
        } else if num < min {
            min = num;
        }
        mean += num as f64;
    }
    mean /= numbers. len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Test passed!");
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * 1.8 + 32.0
}
