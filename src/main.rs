// Rust 의 Type 은 스칼라 , 컴파운드 두개로 나눌수있다.

fn main() {
    //* 스칼라 타입
    //* 부호 있는 정수
    let _x8: i8 = 1; //* 부호 있는 8bit 정수
    let _x16: i16 = 1; //* 부호 있는 16bit 정수
    let _x32: i32 = 1; //* 부호 있는 32bit 정수
    let _x64: i64 = 1; //* 부호 있는 64bit 정수
    let _xany: isize = 1; //* 부호 있는 정수 Dasktop or Hardware 의 Bit 수를 따른다 : 32bit Hardware = 32bit 정수형 / 64Bit for 64Bit 정수

    //* 부호 없는 정수 ( 필수 양수 )
    let _y8: u8 = 1; //* 부호 없는 8bit 정수
    let _y16: u16 = 1; //* 부호 없는 16bit 정수
    let _y32: u32 = 1; //* 부호 없는 32bit 정수
    let _y64: u64 = 1; //* 부호 없는 64bit 정수
    let _yany: usize = 1; //* 부호 없는 정수 Dasktop or Hardware 의 Bit 수를 따른다 : 32bit Hardware = 32bit 정수 / 64Bit for 64Bit 정수

    //* 부동 소수점
    let _float32: f32 = 1.0; //* 32bit 소수
    let _float64: f64 = 1.0; //* 64bit 소수

    //* Boolean
    let _b1: bool = true; //* 1bit Boolean 형 

    //* 문자형
    let _c1:char = 'a'; //* 4bit Unicode 문자 Char Type 은 작은 따옴표
    let _c2 = "String"; //* String 선언시 큰 따옴표

    //* 복합 타입
    //* 튜플
    let tup: (i32, f64, u8) = (1_500, 6.4, 1);
    //* 튜플 구조 해제
    let (_x, _y, _z) = tup; //* 튜플내 변수를 단일 요소로 사용하기위해선 구조해제 시켜서 사용해야한다.

    println!("x = {}", _x);

    //* 튜플 사용시 내부 변수 접근 방법
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!(" f = {} s = {} one = {} ", five_hundred, six_point_four, one);

    //* 배열
    let a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];

    //* 배열은 똑같다.

    //* 정수형 리터럴
    // * Decimal	        98_222
    // * Hex	            0xff
    // * Octal	            0o77
    // * Binary	            0b1111_0000
    // * Byte (u8 only)	    b'A'

    println!("{}", addition(2,6));

    println!("{}", addition_2(8,8));
    //* Block 표현식
    //* 표현식은 세미콜론 미사용
    let block_value = {
        let one_block = 1;
        one_block + one_block + 80
    };
    println!("{}", block_value);

    ifelse();
    loo_p();
}
//* 함수 
//* Function 선언시 Return Type 값 도 필수 지정
fn addition(a:u16, b:u16) -> u16 {
    return a+b;
}
//* return none 형태 함수
//* return 을 사용하지 않을땐 ; 을 붙이지 않아야함 
fn addition_2(a:u16, b:u16) -> u32 {
    a as u32 + b as u32
}

//* if else if else
//* 기존 사용법과 동일 단! () 괄호 제외되어있음
//* let 변수 선언시에도 if 사용가능
fn ifelse() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let num2 = if number > 5 { 5 } else if number < 3 { 2 } else { 6 };
    //* 6 할당됨
    println!("{}", num2);
}
//* 반복문 
//* loop , while , for
//* loop : break 까지 무한반복
//* while : 조건 달성까지 무한 반복
//* for 조건제 반복

fn loo_p() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    for i in 0..10 {
        println!("{}", i);
    }
    let a = [10, 20, 30, 40, 50];
    //* 배열 순회
    for i in a.iter() {
        println!("the value is: {}", i);
    }
    //* 역순으로 순회 
    for i in (9..100).rev() {
        println!("{}", i);
    }
}