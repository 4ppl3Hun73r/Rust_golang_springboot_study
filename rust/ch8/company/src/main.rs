use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
    let mut company_info = HashMap::new();
    loop {
        println!("Input 'add {{name}} to {{dept}}', 'print' or 'exit'");
        let mut cmd = String::new();

        io::stdin().read_line(&mut cmd).expect("Failed to read line!");

        let mut iter = cmd.trim().split_whitespace();

        let cmd1 = iter.next();
        if Some("print") == cmd1 {
            println!("{:?}", company_info);
        } else if Some("exit") == cmd1 {
            break;
        } else if Some("add") == cmd1 {
            let name = iter.next();
            let to = iter.next();
            let dept = iter.next();
            if Some("to") == to && !name.is_none() && !dept.is_none() {
                add_dept(&mut company_info, name.unwrap(), dept.unwrap());
            }
        }
    }
}

fn add_dept(company_info: &mut HashMap<String, Vec<String>>, name: &str, dept: &str) {
    match company_info.entry(String::from(dept)) {
        Entry::Vacant(e) => {e.insert(vec![String::from(name)]);},
        Entry::Occupied(mut e) => {e.get_mut().push(String::from(name));},
    }
}
