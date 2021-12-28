use rand::{self, Rng};

pub fn asciiart() {
    println!("");
    // https://patorjk.com/software/taag/#p=display&f=Small%20Slant&t=BONK%20-%20v0.0.1
    println!("    ___  ____  _  ____ __             ___   ___   ___\n    / _ )/ __ \\/ |/ / //_/ ____  _  __/ _ \\ / _ \\ <  /\n   / _  / /_/ /    / ,<   /___/ | |/ / // // // / / / \n  /____/\\____/_/|_/_/|_|        |___/\\___(_)___(_)_/                                                    ");
    random_tagline();
}

fn random_tagline() {
    let cringy_taglines = [
        "    \"WTF is bonk?\" - Anonymous",
        "    Bonk, a up-and-coming swiss-army knife for playing around with binary files during capture the flag competitions.",
        "    WHY IS BONK SO COOL? BECAUSE IT'S BONK!", // Suggested by Github Copilot
        "    This binary is absolutely bonk.",
    ];
    println!("");
    println!(
        "{}",
        cringy_taglines[rand::thread_rng().gen_range(0..cringy_taglines.len())]
    );
    println!("");
}
