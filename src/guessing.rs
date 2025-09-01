use std::cmp::Ordering;
use std::io; // std (standard) 라이브러리의 io 라이브러리 임포트

use rand::Rng; // rand (random) 라이브러리의 Rng 트레잇 임포트

pub fn main() { // main 함수 선언
    println!("Guess the number!"); // println은 매크로이기 때문에 뒤에 ! 붙이기

    // rand::thread_rng()가 난수 생성기 반환
    // gen_range()로 특정 범위의 난수 생성
    // 1..=100 은 1부터 100을 의미
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop { // 무한반복
        println!("Please input your guess.");

        // let으로 변수 선언, 기본적으로 불변이기 때문에 mut을 붙여 가변으로 지정
        // String은 std에서 제공하는 문자열 타입
        // String::new는 String의 new 함수를 호출한다는 뜻인데, 여기서는 새로운 객체를 생성해 반환
        let mut guess = String::new();

        // 아까 불러온 std::io 라이브러리에서 stdin이라는 함수 호출, std::io::Stdin 반환, 입력을 받는 역할을 함
        io::stdin()
            // 한 줄의 문자열 입력 받기
            // &는 reference를 의미, &mut의 경우에는 reference에 쓸 수 있게 가변으로 제공
            .read_line(&mut guess)
            // read_line에서는 입력받은 reference에 결과를 저장하고 Result 값을 반환함
            // Result 값은 열거형인데, 상태를 반환
            // Result 값은 Ok, Err 등이 있는데, Err인 경우 .expect 안의 msg로 제공된 값을 오류 메시지로 출력
            .expect("Failed to read line");

        // guess를 문자열로 받았는데, 난수와 비교하기 위해서 형변환
        // 불변 unsigned 32-bit number로 guess의 타입을 재지정
        // String 인스턴스의 trim()은 문자열 뒤의 공백을 제거
        // parse()는 지정되는 변수의 타입을 가져와 해당 타입으로 문자열을 변환하고 Result를 반환
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // u32의 형태로 정상적으로 변환되었을 시 num 반환
            Err(_) => continue, // 변환에 실패하였을 때 루프 continue
        };

        // 다른 언어들처럼, println! 안에 변수 삽입 가능
        // println!("{value}");
        // println!("{}", value);
        // 이 두 개의 형태가 다 가능
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // guess 변수를 secret_number와 비교해 일치하는 패턴인지 확인 후 코드 실행
            Ordering::Less => println!("Too small!"), // Ordering::Less의 패턴으로 비교해 패턴에 해당된다면 코드 실행
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 루프 종료
            },
        }
    }
}