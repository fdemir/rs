mod config;
mod exp;

fn main() {
    config::execute();

    let i = 32;
    exp::fizz_buzz::list(&i);

    let mut list = vec![1, 2, 3, 6, 1];
    list.sort();

    println!(
        "Median is = {} Mode is = {}",
        exp::return_median::find_median(&list),
        exp::return_median::find_mode(&list)
    );

    let mut db = exp::employee::Database::new();

    db.add_employee(
        "Engineering".to_string(),
        exp::employee::Employee {
            name: "Furkan".to_string(),
        },
    );

    db.add_employee(
        "Engineering".to_string(),
        exp::employee::Employee {
            name: "Demir".to_string(),
        },
    );

    db.add_employee(
        "Product".to_string(),
        exp::employee::Employee {
            name: "Yellow".to_string(),
        },
    );
    db.list_employees();
    // let mut db = Database::new();
}
