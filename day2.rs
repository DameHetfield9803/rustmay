use std::io;

fn main() {
    print!("Enter your salary (in SGD) > ");

    let mut input_salary = String::new();
    io::stdin().read_line(&mut input_salary).expect("Failed to read input");
    let salary: f32 = input_salary.trim().parse().expect("Invalid salary input");

    print!("Enter your age (>16 and <100 IN INTEGER) as of 2025 > ");

    let mut input_age = String::new();
    io::stdin().read_line(&mut input_age).expect("Failed to read input");
    let age: i8 = input_age.trim().parse().expect("Invalid age input");

    let cpf: f32;
    let new_salary: f32;

    if age >= 18 && age < 55 {
        cpf = salary * 0.37;
        new_salary = salary * 0.80;
    } else if age >= 55 && age < 60 {
        cpf = salary * 0.325;
        new_salary = salary * 0.83;
    } else if age >= 60 && age < 65 {
        cpf = salary * 0.235;
        new_salary = salary * 0.885;
    } else if age >= 65 && age < 70 {
        cpf = salary * 0.165;
        new_salary = salary * 0.925;
    } else if age > 70 {
        cpf = salary * 0.125;
        new_salary = salary * 0.95;
    } else {
        panic!("Age entered is invalid");
    }

    println!("Your cpf will be > {:.2}.\n", cpf);
    println!("Your salary will be > {:.2}.\n", new_salary);
}
