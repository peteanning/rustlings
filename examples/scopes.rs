main() { // made it unsafe because rust-analyzer is complaining about println

    if true {
        let foo = "my string";
    }

    println!("{}", foo);
}

fn sub() {
  let bar = foo;

}
