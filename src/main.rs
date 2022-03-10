use std::io;
extern crate threshold_secret_sharing as tss;

fn main() {
    println!(
        "\nWelcome to the Strategic Air Command Ground Based Strategic Deterrance Launch System\n"
    );

    println!("PEACE IS OUR PROFESSION\n");

    println!("Enter a valid LAUNCH Key:");

    let mut secret = String::new();

    io::stdin()
        .read_line(&mut secret)
        .expect("Failed to read line");

    let secret: i64 = secret.trim().parse().expect("Turn your KEY, SIR!");

    let ref tss = tss::shamir::ShamirSecretSharing {
        threshold: 9,
        share_count: 20,
        prime: 41, // any large enough prime will do
    };

    let all_shares = tss.share(secret);

    let reconstruct_share_count = 10;
    assert!(reconstruct_share_count >= tss.reconstruct_limit());

    let indices: Vec<usize> = (0..reconstruct_share_count).collect();
    let shares: &[i64] = &all_shares[0..reconstruct_share_count];
    let recovered_secret = tss.reconstruct(&indices, shares);

    println!("The recovered secret is {}", recovered_secret);
    println!("The shares are {:#?}", shares);
    assert_eq!(recovered_secret, secret);
}
