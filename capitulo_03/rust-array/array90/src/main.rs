fn main() {
    let origin = [
        [01, 02, 03],
        [04, 05, 06],
        [07, 08, 09]];

    let mut result = origin.clone();
    let len = origin.len();
    
    for linha in 0..len {
        for coluna in 0..len {
            result[linha][coluna] = origin[len-coluna-1][linha];
        }
    }

    println!("## ARRAY ORIGINAL");
    for linha in origin.iter() {
        println!("{:?}", linha);
    }

    println!("## RESULTADO 90");
    for item in result.iter() {
        println!("{:?}", item);
    }
}

/*
## ARRAY ORIGINAL
[1, 2, 3]
[4, 5, 6]
[7, 8, 9]
## RESULTADO 90
[7, 4, 1]
[8, 5, 2]
[9, 6, 3]
*/