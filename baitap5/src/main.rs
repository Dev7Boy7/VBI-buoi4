// CACH 1 ----------------------------------------------------------------
// fn fibo (x: u32) -> u32 {
//     match x {
//         0 => 1,
//         1 => 1,
//         _ => fibo(x-1) + fibo(x-2),
//     }
// }
// fn main() {
//     let mut x = 0;

//     loop {
//         println!("{}", fibo(x));
//         x+=1;
//     }
// } 

//CACH 2 ------------------------------------------------------------------

#[derive(Debug)]
struct Fibonacci {
    a: u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    // Trả về số fibonaci tiếp theo dựa trên kiểu dữ liệu struct Fibonacci
    fn next(&mut self) -> Option<u32> {
        *self = Fibonacci { a: self.b , b : self.a + self.b};
        Some(self.a)
    }
}

    // Khởi tạo ban đầu cho Fibonaci: 0, 1
    fn fibonacci_numbers() -> Fibonacci {
        Fibonacci { a: 1, b: 0 }
}

fn main() {
//     Vì struct Fibonacci có implement trait Iterator của Rust nên 
// có thể dùng câu lệnh for dc
// Câu lệnh for bản chất sẽ chuyển qua trait Iterator nên instance của
// struct Fibonacci có thể duyệt được, 
// Mỗi lần duyệt sẽ tự động chạy function signature next() trên
// Nên cần implement hàm next() cho struct Fiboncci.
    for number in fibonacci_numbers() {
        println!("{}", number);
    }
}