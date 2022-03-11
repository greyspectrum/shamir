use clap::Parser;
use std::io;
extern crate threshold_secret_sharing as tss;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    launchcode: i64,
}

fn main() {

    // The shamir secret to be split must be passed 
    // in as and argument
    let args = Args::parse();

    let launchcode = args.launchcode;

    let tss = tss::shamir::ShamirSecretSharing {
        threshold: 2,
        share_count: 20,
        prime: 41, // any large enough prime will do
    };

    let all_shares = tss.share(launchcode);

    let reconstruct_share_count = 10;
    assert!(reconstruct_share_count >= tss.reconstruct_limit());

    // Reconstruct the secret from the shares
    let indices: Vec<usize> = (0..reconstruct_share_count).collect();
    let shares: &[i64] = &all_shares[0..reconstruct_share_count];
    let recovered_launchcode = tss.reconstruct(&indices, shares);

    println!("\nThe LAUNCH Keys are {:#?}", shares);
    assert_eq!(recovered_launchcode, launchcode);

    // A friendly welcome
    println!(
        "\nWelcome to the Strategic Air Command Ground Based Strategic Deterrance Launch System\n"
    );
    println!("PEACE IS OUR PROFESSION\n");
    println!("Enter a valid LAUNCH Key:");

    // Read the shares in as user input
    let mut key1 = String::new();

    io::stdin()
        .read_line(&mut key1)
        .expect("Failed to read line");

    let key1: i64 = key1.trim().parse().expect("Turn your KEY, SIR!");

    // Determine if the shares are in the set of shares
    // Success is faked here
    if shares.contains(&key1) {
        println!("Enter a valid LAUNCH Key:");
        let mut key2 = String::new();

        io::stdin()
            .read_line(&mut key2)
            .expect("Failed to read line");

        let key2: i64 = key2.trim().parse().expect("Turn your KEY, SIR!");

        if shares.contains(&key2) {
            println!("\nLAUNCH IS AUTHORIZED");
        } else {
            println!("\nAUTHENTICATION FAILED");
        }
    } else {
        println!("\nAUTHENTICATION FAILED");
    }
}
