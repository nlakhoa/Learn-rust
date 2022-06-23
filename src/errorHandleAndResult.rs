struct Employee {
    position: Position,
    status: Status,
}

enum Position {
    IT,
    CEO,
    CTO,
    Manager,
    Marketer,
}

enum Status {
    Active,
    Denied,
}

fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Denied => return Err("Access Denied".to_string()),
        _ => (),
    }

    match employee.position {
        Position::IT => Ok(()),
        Position::CEO => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("invalid position".to_string()),
    }
}

fn main() {
    let manager = Employee {
        position: Position::Manager,
        status: Status::Active,
    };

    let it = Employee {
        position: Position::IT,
        status: Status::Denied,
    };
    let access = try_access(&manager);
    let accesss = try_access(&it);
    println!("{:?}", access);
    println!("{:?}", accesss);
}
