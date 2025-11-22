fn main() {

    // PARTE 1: Variáveis imutáveis e mutáveis
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    let y = 10;
    println!("The value of y (the imutable variable) is: {y}");

    println!("=========================================================");

    // PARTE 2: Constantes
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("3 hours are {THREE_HOURS_IN_SECONDS} seconds long.");

    println!("=========================================================");

    // PARTE 3: shadowing
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is {z}");
    }

    println!("The value of z is {z}");

    println!("=========================================================");

    // PARTE 4: shadowing vs mutabilidade
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The user wants {spaces} spaces");

    let mut spaces = "       ";
    spaces = spaces.len(); // isso não é permitido porque altera o tipo da variável
}
