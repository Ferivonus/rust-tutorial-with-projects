// Bir fonksiyonu argüman olarak alan bir yüksek seviye fonksiyon
fn apply_function(value: i32, func: Box<dyn Fn(i32) -> i32>) -> i32 {
    func(value)
}

// Başka bir fonksiyon döndüren bir fonksiyon
fn create_multiplier(factor: i32) -> Box<dyn Fn(i32) -> i32> {
    let multiplier = move |value: i32| -> i32 {
        value * factor
    };
    Box::new(multiplier)
}

fn main() {
    let result = apply_function(5, Box::new(|x| x * x)); // Girdiyi karesini alan bir closure ile apply_function'ı çağır
    println!("Result: {}", result);

    let multiply_by_3 = create_multiplier(3); // 3 ile çarpan bir fonksiyon elde etmek için create_multiplier'ı çağır
    let result = multiply_by_3(7); // Elde edilen fonksiyonu 7 ile çağır
    println!("Result: {}", result);
}

/*
    Output of this code:

    Result: 25
    Result: 21
*/