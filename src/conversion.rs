// Атрибут, который убирает предупреждения компилятора о неиспользуемом коде
#![allow(dead_code)]
#![allow(unused_variables)]
// 6. Преобразование
/*
Примитивные типы могут быть сконвертированы в другие при помощи приведения типов.

Rust предоставляет преобразование между пользовательскими типами (такими как, struct и enum) через использование трейтов.
Общие преобразования используют трейты From и Into.
Однако есть и конкретные трейты для более частных случаев, например для конвертации String.
*/

// 6.1 From и Into
/*
Типажи From и Into связаны по своей сути, и это стало частью их реализации.
Если вы можете конвертировать тип А в тип В, то будет легко предположить, что мы должны быть в состоянии конвертировать тип В в тип А.
*/

/*
From

Типаж From позволяет типу определить, как он будет создаваться из другого типа, что предоставляет очень простой механизм конвертации между несколькими типами.
Eсть несколько реализаций этот типажа в стандартной библиотеке для преобразования примитивов и общих типов.
let my_str = "привет";
let my_string = String::from(my_str);
*/

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn from() {
    let num = Number::from(30);
    println!("Мой номер {:?}", num);
    // >> Мой номер Number { value: 30 }
}

/*
Into

Трейт Into является полной противоположностью трейта From.
Так что если вы реализовали для вашего типа трейт From, то трейт Into вызовет его при необходимости.
Использование типажа Into обычно требует спецификации типа, в который мы собираемся конвертировать, так как компилятор чаще всего не может это вывести.
Однако это небольшой компромисс, учитывая, что данную функциональность мы получаем бесплатно.
*/

fn into() {
    let int = 5;
    let num: Number = int.into();

    println!("Мой номер {:?}", num);
    // >> Мой номер Number { value: 5 }
}

// 6.2 TryFrom и TryInto
/*
Как и From и Into, TryFrom и TryInto - обобщённые типажи для конвертации между типами.
Но в отличии от From/Into, типажи TryFrom/TryInto используются для преобразований с ошибками и возвращают Result.
*/

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn try_from_try_into() {
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)), "vfvf");
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

// 6.3 FromStr и ToString
/*
Конвертация в строку

Преобразовать любой тип в String так же просто, как и реализовать для него типаж ToString.
Вместо того, чтобы делать это напрямую, вы должны реализовать типаж fmt::Display,
    который автоматически предоставляет реализацию ToString, а также позволяет распечатать тип.
*/

use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Круг радиусом {}", self.radius)
    }
}

fn to_string() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}

/*
Парсинг строки

Один из наиболее общих типов конвертации - это преобразование строки в число.
Идиоматический подход это сделать при помощи функции parse и указания типа, в который будем преобразовывать,
    что можно сделать либо через выведение типа, либо при помощи 'turbofish'-синтаксиса.

Это преобразует строку в указанный тип при условии, что для этого типа реализован типаж FromStr.
Он реализован для множества типов стандартной библиотеки.
Чтобы получить эту функциональность для пользовательского типа, надо просто реализовать для этого типа типаж FromStr.
*/

fn from_str() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Сумма {:?}", sum)
    // >> Круг радиусом 6
}

pub fn run6() {
    from();
    into();
    try_from_try_into();
    to_string();
    from_str();
    // >> Сумма 15
}
