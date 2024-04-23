// Атрибут, который убирает предупреждения компилятора о неиспользуемом коде
#![allow(dead_code)]
#![allow(unused_variables)]

// 3 Пользовательские типы
/*
В языке программирования Rust пользовательские типы данных в основном создаются при помощи двух ключевых слов:
    struct: - определение структуры
    enum - определение перечисления
Константы так же могут быть созданы с помощью ключевых слов const и static.
*/

// 3.1 Структуры

/*
Существует три типа структур, которые можно создать с помощью ключевого слова struct:
    Кортежная структура, которая на самом деле является именованным кортежем.
    Классическая C-структура
    Единичная структура, которая не имеет полей, но может быть полезна для обобщённых типов.
*/

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
// unit-структура
struct Unit;

// Кортежная структура
struct Pair(i32, f32);

// Структура с двумя полями
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Структуры могут быть использованы в качестве полей другой структуры
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        (self.top_left.x - self.bottom_right.x) * (self.top_left.y - self.bottom_right.y)
    }
    fn square(&self, point: Point, width_heidth: f32) -> Rectangle {
        let Point { x, y } = point;
        Rectangle {
            top_left: point,
            bottom_right: Point {
                x: x + width_heidth,
                y: y + width_heidth,
            },
        }
    }
}

fn _struct() {
    let name = String::from("Peter");
    let age = 27;
    // Ключи структуры можно не писать если наименование переменой соответствует ключу структуры
    let peter = Person { name, age };
    println!("{:?}", peter);
    // >> Person { name: "Peter", age: 27 }

    let point = Point { x: 10.3, y: 0.4 };
    println!("Координаты точки: ({}, {})", point.x, point.y);
    // >> Координаты точки: (10.3, 0.4)

    // Создадим новую точку, используя синтаксис обновления структуры и нашу существующую точку
    // `bottom_right.y` будет тем же самым, что и `point.y`, так как мы взяли это поле из `point`
    let bottom_right = Point { x: 5.2, ..point };
    println!("Вторая точка: ({}, {})", bottom_right.x, bottom_right.y);
    // >> Вторая точка: (5.2, 0.4)

    // Деструктурируем структуру при помощи `let`
    let Point {
        x: letf_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        top_left: Point {
            x: letf_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Создадим unit-структуру
    let _unit = Unit;

    // Создадим кортежную структуру
    let pair = Pair(1, 0.1);
    // Доступ к полям кортежной структуры
    println!("Pair cодержит {:?} и {:?}", pair.0, pair.1);
    // >> Pair cодержит 1 и 0.1

    // Деструктурируем кортежную структуру
    let Pair(integer, decimal) = pair;
    println!("Pair содержит {:?} и {:?}", integer, decimal);
    // >> Pair содержит 1 и 0.1

    println!("Площадь прямоугольника: {}", _rectangle.rect_area());
    // >> Площадь прямоугольника: 0
    println!("Квадрат: {:?}", _rectangle.square(point, 13.1));
    // >> Квадрат: Rectangle { top_left: Point { x: 10.3, y: 0.4 }, bottom_right: Point { x: 23.400002, y: 13.5 }
}

// 3.2 Перечисления
/*
Ключевое слово enum позволяет создавать тип данных, который представляет собой один из нескольких возможных вариантов.
Любой вариант, действительный как struct, также действителен как enum.
*/
/*
Создаём `enum` для классификации web-событий. Обратите внимание,
как имена и информация о типе определяют вариант:
`PageLoad != PageUnload` и `KeyPress(char) != Paste(String)`.
Все они разные и независимые.
*/

enum WebEvent {
    // `enum` может быть как `unit-подобным`,
    PageLoad,
    PageUnload,
    // так и кортежной структурой,
    KeyPress(char),
    Paste(String),
    // или С-подобной структурой.
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Страница загружена"),
        WebEvent::PageUnload => println!("Стараница не загружена"),
        WebEvent::KeyPress(c) => println!("Нажата '{}'.", c),
        WebEvent::Paste(s) => println!("Нажата \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("Кликнуто на x={}, y={}", x, y);
        }
    }
}

fn _enum() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` создаст `String` из строкового среза.
    let pasted = WebEvent::Paste("Мой текст".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    // >> Нажата 'x'.
    inspect(pasted);
    // >> Нажата "Мой текст"
    inspect(click);
    // >> Кликнуто на x=20, y=80
    inspect(load);
    // >> Страница загружена
    inspect(unload);
    // >> Стараница не загружена
}
// 3.2.1 Псевдонимы типов
/*
Если вы используете псевдонимы типов, то вы можете обратиться к каждому варианту перечисления через его псевдоним.
Это может быть полезно, если у перечисления слишком длинное имя или оно слишком обобщено, и вы хотите переименовать его.
*/
#[derive(Debug)]
enum VeriVerboseEnumOfThingsDoWithNumber {
    Add,
    Substract,
}
// Создаётся псевдоним типа
type Operations = VeriVerboseEnumOfThingsDoWithNumber;

impl VeriVerboseEnumOfThingsDoWithNumber {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Substract => x - y,
        }
    }
}

fn pseudonym() {
    // Мы можем обратиться к каждому варианту перечисления через его
    // псевдоним, а не через его длинное неудобное имя.
    let x = Operations::Add;
    println!("x: {:?}", x);
    // >> x: Add
    println!("x.run: {}", x.run(10, 20));
    // >> x.run: 30
}

// 3.2.2 Декларация use

// Декларация use используется, чтобы убрать необходимость указывать область видимости

enum Status {
    Rich,
    Poor,
}
enum Work {
    Civilian,
    Solidier,
}

fn use_of_declaration() {
    // Используем `use` для каждого из вариантов, чтобы они были доступны без указания области видимости.
    use Status::{Poor, Rich};
    // Автоматически используем `use` для каждого из вариантов в `Work`.
    use Work::*;

    // Эквивалентно `Status::Poor`.
    let status = Poor;

    // Эквивалентно `Work::Civilian`.
    let work = Civilian;

    match status {
        Rich => println!("У богатого куча денег!"),
        Poor => println!("У бедняка денег нет, но он держится..."),
    }
    // >> У бедняка денег нет, но он держится...

    match work {
        Civilian => println!("Гражданин работате!"),
        Solidier => println!("Солдыты служат!"),
    }
    // >> Гражданин работате!
}

// 3.2.2 С-подобные перечисления
// enum могут быть использованы как С-подобные перечисления.
// enum с неявным дискриминатором (начинается с 0)
enum Number {
    Zero,
    One,
    Two,
    Ten,
}
// enum с явным дискриминатором
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn c_like() {
    // `enums` может быть преобразован в целочисленное значение. (идекс значения от 0)
    println!("Нулевой элемент {}", Number::Zero as i32);
    // >> Нулевой элемент 0
    println!("Первый элемент {}", Number::One as i32);
    // >> Первый элемент 1

    println!("Красный цвет #{:06x}", Color::Red as i32);
    // >> Красный цвет #ff0000
    println!("Голубой цвет #{:06x}", Color::Blue as i32);
    // >> Голубой цвет #0000ff
}

// 3.2.3 Пример: Связанный список
// Импорт всего модуля
use List::*;

enum List {
    // Cons: Кортежная структура, которая хранит элемент и указатель на следующий узел
    Cons(u32, Box<List>),
    // Nil: Узел, обозначающий конец связанного списка
    Nil,
}

// Методы могут быть присоединены к перечислению
impl List {
    // Создаём пустой список
    fn new() -> List {
        // `Nil` имеет тип `List`
        Nil
    }

    // Функция, которая принимает список и возвращает тот же список, но с новым элементом в начале
    fn prepende(self, elem: u32) -> List {
        // `Cons` также имеет тип `List`
        Cons(elem, Box::new(self))
    }

    // Возвращаем длину списка
    fn len(&self) -> u32 {
        /*
        `self` должен быть сопоставлен (проверен на соответствие), поскольку поведение этого метода зависит от варианта `self`
        `self` имеет тип `&List`, а `*self` имеет тип `List`, сопоставление на конкретном типе `T` предпочтительнее, чем сопоставление по ссылке `&T`
        */
        match *self {
            // Мы не можем завладеть `tail`, т.к. `self` заимствован; вместо этого возьмём ссылку на `tail`
            Cons(_, ref tail) => 1 + tail.len(),
            // Базовый случай: Пустой список имеет нулевую длину
            Nil => 0,
        }
    }

    // Возвращаем представление списка в виде (размещённой в куче) строки
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` похож на `print!`, но возвращает строку размещённую в куче, вместо вывода на консоль
                format!("{}, {}", head, tail.stringify())
            }
            Nil => format!("Nil"),
        }
    }
}

fn example_list() {
    // Создаём пустой связанный список
    let mut list = List::new();

    // Присоединяем несколько элементов
    list = list.prepende(1);
    list = list.prepende(2);
    list = list.prepende(3);

    println!("Размер связаного списка {}", list.len());
    // >> Размер связаного списка 3
    println!("{}", list.stringify());
    // >> 3, 2, 1, Nil
}

// 3.3 Константы
/*
В Rust есть два типа констант, которые могут быть объявлены в любой области видимости, включая глобальную. Оба требуют явной аннотации типа:

    const: Неизменяемая переменная (в общем случае).
    static: Возможно, изменяемая переменная с временем жизни 'static. Статическое время жизни подразумевается и может не быть указано явно.
        Доступ или модификация изменяемой статической переменной — небезопасны (см. unsafe).
*/

static LANGUAGE: &str = "Rust";
const THERESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Получаем доступ к константе внутри функции
    n > THERESHOLD
}

fn constants() {
    let n = 16;

    // Получаем доступ к константе внутри функции main
    println!("Это язык {}", LANGUAGE);
    // >> Это язык Rust
    println!("Установим предел равный {}", THERESHOLD);
    // >> Установим предел равный 10
    println!(
        "Число {} {} предела",
        n,
        if is_big(n) {
            "Больше"
        } else {
            "Меньше"
        }
    );
    // >> Число 16 Больше предела
}

pub fn run3() {
    _struct();
    _enum();
    pseudonym();
    use_of_declaration();
    c_like();
    example_list();
    constants();
}
