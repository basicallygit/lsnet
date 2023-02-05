use std::process::Command;

type Connections = Vec<(String, usize)>;

fn fetch_connections() -> Connections { // Vec<(Program, connection count)>
    let output = Command::new("lsof")
        .arg("-i")
        .output()
        .expect("Failed to execute lsof");
    let output = String::from_utf8(output.stdout).unwrap();
    let mut connections: Connections = Vec::new();

    for line in output.lines().skip(1).map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>()) {
        if line.len() < 9 {
            continue;
        }
        let program = line[0].to_string();
        if let Some((_, count)) = connections.iter_mut().find(|(p, _)| p == &program) {
            *count += 1;
        } else {
            connections.push((program, 1));
        }
    }

    connections
}

fn main() {
    let connections = fetch_connections();

    println!("PROGRAM : CONNECTIONS");
    for (program, count) in connections {
        println!("{} : {}", program, count);
    }
}