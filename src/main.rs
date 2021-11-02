use std::process::Command;
use std::io;

fn get_current_time() -> (u8, u8) {

    // --- HOURS ---
    let hours = Command::new("date")
        .arg("+\"%H\"")
        .output()
        .expect("failed to get time (hours)");

    let hours = match String::from_utf8(hours.stdout) {
        Ok(hrs) => hrs,
        Err(_) => String::from(""),
    };

    // Converting to usable form
    let hours: u8 = hours[1..3].to_string().parse::<u8>().unwrap();

    // --- MINUTES ---
    let minutes = Command::new("date")
        .arg("+\"%M\"")
        .output()
        .expect("failed to get time (minutes)");

    let minutes = match String::from_utf8(minutes.stdout) {
        Ok(mins) => mins,
        Err(_) => String::from(""),
    };

    // Converting to usable form
    let minutes: u8 = minutes[1..3].to_string().parse::<u8>().unwrap();

    let hours_mins: (u8, u8) = (hours, minutes);

    hours_mins
}

struct Time {
    hours: u8,
    minutes: u8,
}

struct Application {
    name: String,
    start_time: Time,
    end_time: Time,
}

impl Application {
    fn kill_process(&self) {
        Command::new("pkill")
            .arg("-f")
            .arg(&self.name)
            .output()
            .expect("Failed to kill the app");
    }
}

fn check_time(start_time: &Time, end_time: &Time) -> bool {
    if &start_time.hours > &end_time.hours {
        false
    } else {
        if &start_time.hours == &end_time.hours {
            if &start_time.minutes > &end_time.minutes {
                false
            } else {
                true
            }
        } else {
            true
        }
    }
}

fn exec(apps: &String, end_time: (u8, u8)) {
    let current_time = get_current_time();

    let apps_iter = apps.split(" ");

    for app in apps_iter {
        let application = Application {
            name: String::from(app),

            start_time: Time {
                hours: current_time.0,
                minutes: current_time.1,
            },

            end_time: Time {
                hours: end_time.0,
                minutes: end_time.1,
            },
        };

        if check_time(&application.start_time, &application.end_time) == true {
            application.kill_process();
        } else {
            println!("Done");
            std::process::exit(0);
        }
    }
}

fn main() {
    let mut apps: String = String::new();

    println!("Enter the apps to kill!");

    io::stdin()
        .read_line(&mut apps)
        .expect("failed to read line");

    apps.pop();

    println!("Apps to be killed till~");

    let mut time = String::new();

    io::stdin()
        .read_line(&mut time)
        .expect("failed to read line");

    let time: Vec<&str> = time.trim().split(":").collect();
    let hours = time[0].parse::<u8>().unwrap();
    let mins = time[1].parse::<u8>().unwrap();

    let hours_mins: (u8, u8) = (hours, mins);

    loop {
        Command::new("sleep")
            .arg("5")
            .output()
            .expect("Failed to sleep the thread");

        exec(&apps, hours_mins);
    }
}