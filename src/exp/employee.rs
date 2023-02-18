use std::collections::HashMap;

#[derive(Debug)]
pub struct Employee {
    pub name: String,
}

pub struct Database {
    departments: HashMap<String, Vec<Employee>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            departments: HashMap::new(),
        }
    }

    pub fn add_employee(&mut self, department: String, employee: Employee) {
        self.departments
            .entry(department)
            .or_insert(Vec::new())
            .push(employee);
    }

    pub fn list_employees(&self) {
        println!("{:?}", self.departments)
    }
}
