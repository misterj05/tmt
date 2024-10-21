use rand::Rng;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

pub fn move_cursor() {
    let mut child = Command::new("dotool")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn dotool process");

    let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stdout_reader = BufReader::new(stdout);

    let stdout_handle = thread::spawn(move || {
        for line in stdout_reader.lines() {
            match line {
                Ok(line) => println!("Child output: {}", line),
                Err(e) => eprintln!("Error reading child stdout: {}", e),
            }
        }
    });

    let mut rng = rand::thread_rng();

    let target_x: i32 = loop {
        let x = rng.gen_range(-500..=500);
        if x < -150 || x > 150 {
            break x;
        }
    };

    let target_y: i32 = loop {
        let y = rng.gen_range(-300..=300);
        if y < -150 || y > 150 {
            break y;
        }
    };

    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;

    thread::sleep(Duration::from_secs(1));
    println!("Moving towards target X: {}, Y: {}", target_x, target_y);

    while current_x != target_x || current_y != target_y {
        let mut move_x = 0;
        let mut move_y = 0;

        let distance_x = (target_x - current_x).abs();
        let distance_y = (target_y - current_y).abs();

        let increment = if distance_x <= 3 && distance_y <= 3 {
            1
        } else {
            rng.gen_range(1..=3)
        };

        if current_x < target_x {
            move_x = increment;
        } else if current_x > target_x {
            move_x = -increment;
        }

        if current_y < target_y {
            move_y = increment;
        } else if current_y > target_y {
            move_y = -increment;
        }

        current_x += move_x;
        current_y += move_y;

        let command = format!("mousemove {} {}\n", move_x, move_y);
        child_stdin
            .write_all(command.as_bytes())
            .expect("Failed to write to stdin");

        println!("Sent: {}", command.trim());

        thread::sleep(Duration::from_millis(rng.gen_range(10..=15)));
    }

    println!("Reached target X: {}, Y: {}", target_x, target_y);

    drop(child_stdin);

    let output_status = child.wait().expect("Failed to wait on child process");

    stdout_handle.join().expect("Failed to join stdout thread");

    println!("Child exited with status: {}", output_status);
}

pub fn scroll_wheel() {
    let mut child = Command::new("dotool")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn dotool process");

    let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");

    let mut rng = rand::thread_rng();

    let target_value: i32 = loop {
        let value = rng.gen_range(-20..=20);
        if value < -10 || value > 10 {
            break value;
        }
    };

    let mut current_value = 0;

    while current_value != target_value {
        let increment = if current_value < target_value { 1 } else { -1 };

        let command = format!("wheel {}\n", increment);
        child_stdin
            .write_all(command.as_bytes())
            .expect("Failed to write to stdin");

        println!("Sent: {}", command.trim());

        current_value += increment;

        thread::sleep(Duration::from_millis(rng.gen_range(50..=100)));
    }

    drop(child_stdin);

    let output_status = child.wait().expect("Failed to wait on child process");

    println!("Child exited with status: {}", output_status);
}
