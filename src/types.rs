pub fn main(){
    let x = 5;

    // let의 경우에는 불변이어도 변수 덮어쓰기가 가능함
    let x = x + 1; // 6

    {
        // 중괄호로 한번 감싸면 shadowing이 됨
        // shadowing이 된 변수는 중괄호 바깥에서는 변경점이 나타나지 않음
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12 출력
    }

    println!("The value of x is: {x}"); // 6 출력

    let spaces = "     ";
    let spaces = spaces.len();
    // let mut spaces = "     ";
    // spaces = spaces.len();
    // 이 경우에는 spaces의 타입이 달라지기 때문에 불가능함

    println!("The length of spaces is: {spaces}");

    const CONSTANT: &str = "Hello";
    // const CONSTANT: &str = "World"; const는 같은 변수를 여러번 정의할 수 없음
    const INTEGER: u32 = 10; // 문자열은 &str인데 왜 숫자는 &를 안붙여도 되지???
    println!("String: {CONSTANT}\nInteger: {INTEGER}");
    /*
    숫자는 타입 지정할때 그대로 쓰는데 왜 문자열만 타입 지정할때 앞에 포인터(&)를 붙일까?

    숫자와 같이 길이가 미리 지정된 변수는 Sized 변수라 부름.
    이 경우에는 길이를 알 수 있기 때문에 변수에 저장해도 아무 문제가 없음.
    그러나 문자열은 말 그대로 문자의 배열이기 때문에 문자 하나하나의 크기는 알 수 가 있어도 문자열의 길이는 모름.
    이런 이유 때문에 문자열은 길이를 알 수 없는 Unsized 변수임.
    따라서 우리는 문자열을 읽기 전용 메모리에 써놓고 앞에 포인터(&)를 붙여서 사용함.
    포인터는 메모리 주소, 길이를 제공하기 때문에 그제서야 길이를 알 수 있음.
    따라서 타입 지정을 할 때 str 대신 &str을 사용해야 길이를 알 수 있기 때문에 &str을 사용함.
     */

    let decimal: i32 = 123_456; // 123456 언더바는 쉼표 느낌으로 변수 지정에 아무 상관 없음
    let hex: i32 = 0x8a;
    let octal: i32 = 0o77;
    let binary: i32 = 0b1101_1011;
    let byte: u8 = b'A'; // 한글자만 가능
    println!("Decimal: {decimal}\nHex: {hex}\nOctal: {octal}\nBinary: {binary}\nByte: {byte}"); // 모두 10진수로 변환되어 출력됨

    // float은 모두 signed
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("f64: {y}\nf64: {y}");

    let sum1 = 5 + 10; // unsigned integer
    let sum2 = 9.0 + 10.0; // float
    // let sum3 = 7 + 8.0
    // 서로 타입이 다르면 덧셈 불가
    println!("int + int = {sum1}\nfloat + float = {sum2}");

    let sub1 = 5 - 10;
    let sub2 = 9.0 - 1.0;
    println!("int - int = {sub1}\nfloat - float = {sub2}");

    let mul1 = 5 * 6;
    let mul2 = 9.0 / 2.0;
    println!("int * int = {mul1}\nfloat * float = {mul2}");

    let div1 = 31 / 7;
    let div2 = 31.0 / 7.0;
    println!("int / int = {div1} (truncated)\nfloat / float = {div2} (quotient)");

    let rem = 43 % 5;
    println!("int % int = {rem}");

    let c = 'z';
    let z = 'ℤ';
    let emoji = '😻';

    println!("{c}{z}{emoji}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("{x} {y} {z}\n{five_hundred}\n{six_point_four}\n{one}");
}