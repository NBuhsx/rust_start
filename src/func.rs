// 9. Функции
/*
Функции объявляются с помощью ключевого слова `fn`.
Их аргументы имеют явно заданный тип, как у переменных,
    и, если функция возвращает значение, возвращаемый тип должен быть указан после стрелки ->.

Последнее выражение в функции будет использовано как возвращаемое значение.
Так же можно использовать оператор return,
    чтобы вернуть значение из функции раньше, даже из цикла или оператора if.
*/

// Функция, которая возвращает булево значение
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Особый случай, ранний возврат
    if rhs == 0 {
        return false;
    }
    // Это выражение, ключевое слово `return` здесь не требуется
    lhs % rhs == 0
}

// Функции которые "не" возвращают значение, на самом деле возвращают единичный тип `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n)
    }
}
// Если функция возвращает `()`, тип возвращаемого значения можно не указывать в сигнатуре
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

fn func() {
    fizzbuzz_to(100);
}

// 9.1 Методы
/*
Некоторые функции связаны с определённым типом.
Они бывают двух видов: связанные функции и методы.
Связанные функции — это функции, которые обычно определены для типа в целом.
Методы — это связанные функции, которые вызываются для конкретного экземпляра типа.
*/

struct Point {
    x: f64,
    y: f64,
}

// Блок реализации, все функции и методы, связанные с типом `Point` размещаются здесь
impl Point {
    /*
    Это "связанная функция", так как эта функция связана с конкретным типом, в данном случае, Point.
    Связанные функции не обязательно вызывать с каким-то экземпляром класса.
    Чаще всего такие функции используются как конструкторы.
    */
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    // Ещё одны связанная функция, принимающая два аргумента:
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    /*
    Это метод
    `&self` - это синтаксический сахар для замены `self: &Self`, где `Self` - это тип вызывающего объекта
    В данном случае `Self` = `Rectangle`
    */
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        // `abs` - это метод, возвращающий переменную типа `f64`, равную абсолютному значению вызывающего объекта
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 + y2).abs())
    }

    /*
    Этот метод требует, чтобы вызывающий объект был изменяемым
    `&mut self` преобразуется в  `self: &mut Self`
    */
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` владеет ресурсами: двумя целыми числами, память для которых выделена в куче
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    /*
    Этот метод "потребляет" ресурсы вызывающего объекта `self` преобразуется в `self: Self`
    */
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Удаляем Pair({}, {})", first, second);
    }
}

fn method() {
    let rectangle = Rectangle {
        // Связанные функции вызываются с помощью двойных двоеточий
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    /*
    Методы вызываются с помощью оператора "точка"
    Обратите внимание, что первый аргумент `&self` передаётся неявно, т.е.
        `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    */
    println!("Периметр прямоугольника: {}", rectangle.perimeter());
    // >> Периметр прямоугольника: 14
    println!("Площадь прямоугольника: {}", rectangle.area());
    // >> Площадь прямоугольника: 12

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Ошибка! `rectangle` неизменяемый, но в методе требуется изменяемый объект
    // rectangle.translate(1.0, 1.0);

    // Порядок! Изменяемые объекты могут вызывать изменяемые методы
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();
    // Ошибка! Предыдущий вызов `destroy` "употребил" переменную `pair`
    // pair.destroy();
}

// 9.2 Замыкание
/*
Замыкания в Rust, так же называемые лямбда, это функции, которые замыкают своё окружение.
Для примера, замыкание, которое захватывает значение переменной x:
    `|val| val + x`
Синтаксис и возможности замыканий делают их очень удобными для использования "на лету".
Использование замыканий похоже на использование функций.
Однако, тип входных и возвращаемых значений может быть выведен, а название аргумента должно быть указано.

Другие характеристики замыканий включают в себя:
    использование || вместо () для аргументов.
    опциональное ограничения тела функции ({}) для одного выражения (в противном случае обязательно).
    возможность захвата переменных за пределами окружения

*/

fn circuit() {
    // Инкремент с помощью замыкания и функции.
    fn funtion(i: i32) -> i32 {
        i + 1
    }

    /*
    Замыкания анонимны. Тут мы связываем их с ссылками
    Аннотация идентичны аннотации типов функции, но является опциональной
        как и оборачивания тела в `{}`.
    Эти безымянные функции назначены соответствующе названным переменным.
    */
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;

    println!("Функция: {}", funtion(i));
    // >> Функция: 2
    println!("Замыкание с указанием типа: {}", closure_annotated(i));
    // >> Замыкание с указанием типа: 2
    println!("Замыкание с выводом типа: {}", closure_inferred(i));
    // >> Замыкание с выводом типа: 2

    let one = || 1;
    println!("Замыкание, возращает один: {}", one());
    // >> Замыкание, возращает один: 1
}

// 9.2.1 Захват
/*
Замыкания довольно гибкие и делают всё, что требуется, для работы с ними без дополнительных указаний.
Это позволяет захватывать переменные гибко, перемещая их или заимствуя, в зависимости от ситуации.
Замыкания могут захватывать переменные:
    по ссылке: &T
    по изменяемой ссылке: &mut T
    по значению: T
Преимущественно, они захватывают переменные по ссылке, и используют другие способы только там, где это необходимо.
*/

fn circuit_capture() {
    use std::mem;

    let color = "green";
    /*
    Замыкание для вывода `color`, которое немедленно заимствует (`&`)
        `color` и сохраняет замыкание в переменной `print`. `color` будет оставаться
    заимствованным до тех пор, пока `print` не будет использован в последний раз.
    `println!` принимает аргументы только по неизменяемым ссылкам,
        поэтому он не накладываетдополнительных ограничений.
    */

    let print = || println!("color: {}", color);
    // Вызывает замыкание, использующее заимствование.
    print();
    // >> color: green

    // `color` может быть неизменяемо заимствован, так как замыкание держит то только неизменяемую ссылку на `color`.
    let _reborrow = &color;
    print();
    // >> color: green

    // Перемещение или перезанятие возможно после последнего использования `print`
    let _color_moved = color;

    let mut count = 0;
    /*
    Замыкание для увеличения `count` может принимать как `&mut count`, так и `count`,
        но использование `&mut count` накладывает меньше ограничений,
        так что замыкание выбирает первый способ, т.е. немедленно заимствует `count`.
        `inc` должен быть `mut`, поскольку внутри него хранится `&mut`.
    Таким образом, вызов замыкания изменяет его, что недопустимо без `mut`.
    */
    let mut inc = || {
        count += 1;
        println!("count: {}", count)
    };
    for _ in 1..5 {
        /*
        Вызываем замыкание, использующее изменяемое заимствование.
        Замыкание продолжает изменяемо заимствовать `count`, так как оно используется дальше.
        Попытка перезанять приведёт к ошибке.
        // let _reborrow = &count;
        // ^ TODO: попробуйте раскомментировать эту строку.
        */
        inc();
        // Попытка перезанять приведёт к ошибке.
        // let _reborrow = &count;
    }

    /*
    Замыкание больше не заимствует `&mut count`.
    Так что теперь при перезаимствовании ошибок не будет.
    */
    let _count_reborrweb = &mut count;

    // Некопируемый тип.
    let movable = Box::new(3);

    /*
    `mem::drop` требует `T`, так что захват производится по значению.
    Копируемый тип будет скопирован в замыкание, оставив оригинальное значение без изменения.
    Некопируемый тип должен быть перемещён, так что movable` немедленно перемещается в замыкание.
    */
    let consume = || {
        println!("moveble: {:?}", movable);
        mem::drop(movable);
    };
    // `consume` поглощает переменную, так что оно может быть вызвано только один раз.
    consume();
    // Ошибка так как переменной `movable` уже не сущществует
    // consume();
    // println!("{:?}", movable);

    /*
    Использование move перед вертикальными линиями позволяет получить владение над захваченными переменными:
    */
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    // >> true
    println!("{}", contains(&7));
    // >> false

    // !Ошибка `move` передало владение в функцию. `haystack` больше не существует
    // println!("Количество элементов {} в векторе", haystack.len());
    // >> Количество элементов 3 в векторе
}

// 9.2.2 Как входные параметры
/*
В то время как замыкания Rust выбирают способ захвата переменных на лету, по большей части без указания типов,
    эта двусмысленность недопустима при написании функций.
При использовании замыкания в качестве входного параметра, его тип должен быть указан с использованием одного из типажей.
Вот они, в порядке уменьшения ограничений:
    Fn: замыкание захватывает по ссылке (&T)
    FnMut: замыкание захватывает по изменяемой ссылке (&mut T)
    FnOnce: замыкание захватывает по значению (T)
Компилятор стремится захватывать переменные наименее ограничивающим способом.
Для примера, рассмотрим аргумент, указанный как FnOnce.
Это означает, что замыкание может захватывать &T, &mut T, или T, но компилятор в итоге будет выбирать в зависимости от того,
    как захваченные переменные используются в замыкании.

Это связано с тем, что если перемещение возможно, тогда любой тип заимствования также должен быть возможен.
Отметим, что обратное не верно. Если параметр указан как Fn, то захват переменных как &mut T или T недопустим.
*/

/*
Функция, которая принимает замыкание в качестве аргумента и вызывает его.
<F> обозначает, что F - "параметр общего типа"
*/
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    // Замыкание ничего не принимает и не возвращает.
    f();
}

// Функция, которая принимает замыкание и возвращает `i32`.
fn apply_to_3<F>(f: F) -> i32
where
    // Замыкание принимает `i32` и возвращает `i32`.
    F: Fn(i32) -> i32,
{
    f(3)
}

fn input_param_circuit() {
    use std::mem;

    let greeting = "привет";
    // Не копируемый тип. `to_owned` преобразует заимствованные данные в собственные.
    let mut farawell = "пока".to_owned();

    // Захват двух переменных: `greeting` по ссылке и `farewell` по значению.
    let diary = || {
        // `greeting` захватывается по ссылке: требует `Fn`.
        println!("Я сказал {}.", greeting);
        // Изменяемость требует от `farewell` быть захваченным по изменяемой ссылке. Сейчас требуется `FnMut`.
        farawell.push_str("!!!");

        println!("Потом я закричал {}.", farawell);
        println!("Теперь я могу поспать. zzzzz");

        // Ручной вызов удаления требуется от `farewell` быть захваченным по значению. Теперь требуется `FnOnce`.
        mem::drop(farawell);
    };
    // Вызов функции, которая выполняет замыкание.
    apply(diary);

    // `double` удовлетворяет ограничениям типажа `apply_to_3`
    let double = |x| 2 * x;
    println!("Удвоенное 3: {}", apply_to_3(double));
}

// 9.2.3 Анонимность типов
/*
Замыкания временно захватывают переменные из окружающих областей видимости.
Имеет ли это какие-либо последствия? Конечно.
Как видите, использование замыкания в аргументах функции требует обобщённых типов, из-за особенностей реализации замыканий:
    `F` должен быть обобщённым типом.
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}
Когда компилятор встречает определение замыкания, он неявно создаёт новую анонимную структуру для хранения захваченных переменных,
тем временем реализуя функциональность для этого неизвестного типа,
с помощью одного из типажей: Fn, FnMut, или FnOnce.
Этот тип присваивается переменной, которая хранится до самого вызова замыкания.
Так как этот новый тип заранее неизвестен, любое его использование в функции потребует обобщённых типов.
Тем не менее, неограниченный параметр типа <T> по прежнему будет неоднозначным и недопустимым.
Таким образом, ограничение по одному из типажей: Fn, FnMut, или FnOnce (которые он реализует) является достаточным для указания этого типа.
*/

/*
`F` должен реализовать `Fn` для замыкания, которое ничего не принимает и не возвращает - именно то, что нужно для `print`.
fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}
*/

fn annotated_circuit() {
    let x = 7;
    // Захватываем `x` в анонимный тип и реализуем `Fn` для него. Сохраняем его как `print`.
    let print = || println!("{}", x);
    apply(print);
}

// 9.2.4 Входные функции
/*
Так как замыкания могут использоваться в аргументах, вы можете ожидать, что то же самое можно сказать и про функции.
И это действительно так! Если вы объявляете функцию, принимающую замыкание как аргумент,
то любая функция, удовлетворяющая ограничениям типажа этого замыкания, может быть передана как аргумент.
*/

//Объявляем функцию, которая принимает обобщённый тип `F`, ограниченный типажом `Fn`, и вызывает его.
fn call_me<F: Fn()>(f: F) {
    f();
}

// Объявляем функцию-обёртку, удовлетворяющую ограничению `Fn`
fn function() {
    println!("Я функция!");
}

fn output_func_circuit() {
    // Определяем замыкание, удовлетворяющее ограничению `Fn`
    let closure = || println!("Я замыкание");

    call_me(closure);
    call_me(function);
    // Стоит отметить, что типажи Fn, FnMut и FnOnce указывают, как замыкание захватывает переменные из своей области видимости.
}

// 9.2.5 Как выходные параметры
/*
Замыкания могут выступать как в качестве входных параметров, так и в качестве выходных.
Однако тип анонимных замыканий по определению не известен, из-за чего для их возврата нам придётся использовать `impl Trait`.
Для возврата замыкания мы можем использовать такие трейты:
    Fn
    FnMut
    FnOnce
Помимо этого, должно быть использовано ключевое слово `move`, чтобы сигнализировать о том, что все переменные захватываются по значению.
Это необходимо, так как любые захваченные по ссылке значения будут удалены после выхода из функции, оставляя недопустимые ссылки в замыкании.
*/

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("a: {}", text)
}

fn output_params_circuit() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

// 9.2.6 Примеры замыканий из стандарной библиотеке std
// 9.2.6.1 Iterator::any
/*
Iterator::any - это функция, которая принимает итератор и возвращает true, если любой элемент удовлетворяет предикату. Иначе возвращает false.
*/

/*
pub trait Iterator {
    // Тип, по которому выполняется итерирование
    type Item;

    // `any` принимает `&mut self`, что означает заимствование и изменение, но не поглощение `self`.
    fn any<F>(&mut self, f: F) -> bool
    where
        /*
        `FnMut` означает, что любая захваченная переменная
        может быть изменена, но не поглощена. `Self::Item`
        указывает на захват аргументов замыкания по значению.
        */
        F: FnMut(Self::Item) -> bool,
    {
    }
}
*/

fn example_iterator_any() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` для векторов даёт `&i32`. Приводим к `i32`.
    println!("2 в vec1: {}", vec1.iter().any(|&x| x == 2));
    // >> 2 в vec1: true

    // `into_iter()` для векторов даёт `i32`. Приведения не требуется.
    println!("2 в vec2: {}", vec2.into_iter().any(|x| x == 2));
    // >> 2 в vec2: false

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` для массивов даёт `&i32`.
    println!("2 в array1: {}", array1.iter().any(|&x| x == 2));
    // >> 2 в array1: true

    // `into_iter()` для массивов даёт `i32`. Приведения не требуется.
    println!("2 в array2: {}", array2.into_iter().any(|x| x == 2));
    // >> 2 в array2: false
}

// 9.2.6.2 Поиск через итераторы

pub trait Iterator {
    // Тип, по которому выполняется итерирование.
    type Item;

    /*
    `find` принимает `&mut self`, что означает,
    что вызывающий объект может быть заимствован и изменён, но не поглощён.
    */
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        /*
        `FnMut` означает, что любая захваченная переменная может максимум быть
            изменена, но не поглощена. `&Self::Item`
            казывает на то, что аргументы замыкания берутся по ссылке.
        */
        P: FnMut(&Self::Item) -> bool;
}

fn example_iterator_find() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` для векторов выдаёт `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` для векторов выдаёт `i32`.
    let mut into_iter = vec2.into_iter();

    /*
    `iter()` для векторов выдаёт `&i32`, а мы хотим ссылаться на один из его элементов,
    поэтому нам нужно деструктурировать `&&i32` в `i32`
    */
    println!("Найдём 2 в vec1: {:?}", iter.find(|&&x| x == 2));
    // >> Найдём 2 в vec1: Some(2)

    /*
    `into_iter()` для векторов выдаёт `i32`, а мы хотим ссылаться на один из его элементов,
    поэтому нам нужно деструктурировать `&i32` в `i32`
    */
    println!("Найдём 2 в vec2: {:?}", into_iter.find(|&x| x == 2));
    // >> Найдём 2 в vec2: None

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` для массивов выдаёт `&i32`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // >> Find 2 in array1: Some(2)

    // `into_iter()` для массивов выдаёт `i32`
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );
    // >> Find 2 in array2: None

    let vec = vec![1, 9, 3, 3, 13, 2];

    /*
    `iter()` для векторов выдаёт `&i32`, а `position()` не принимает ссылку,
    поэтому мы должны деструктурировать `&i32` в `i32`
    */
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    /*
    `into_iter()` для векторов выдаёт `i32`, а `position()` не принимает ссылку,
    поэтому деструктуризация не требуется
    */
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}

// 9.3 Функции высшего порядка
/*
Rust предоставляет Функции Высшего Порядка (ФВП).
Это функции, которые получают на вход одну или несколько функций и/или выдают более полезную функцию.
ФВП и ленивые итераторы придают языку Rust функциональный оттенок.
*/

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn func_fvp() {
    println!("Найти сумму всех квадватов нечётных чисел не больше 1000");
    let upper = 1000;

    // Императивный подход. Объявляем переменную-накопитель
    let mut acc = 0;

    // Итерируем: 0, 1, 2, ... до бесконечности
    for n in 0.. {
        // Возводим число в квадрат
        let n_squared = n * n;
        if n_squared >= upper {
            // Останавливаем цикл, если превысили верхний лимит
            break;
        } else if is_odd(n_squared) {
            // Прибавляем число, если оно нечётное
            acc += n_squared;
        }
    }
    println!("Императивный стиль: {}", acc);
    // >> Императивный стиль: 5456

    // Функциональный подход
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // Все натуральные числа возводим в квадрат
        .take_while(|&n_squared| n_squared < upper) // Берём те, что ниже верхнего предела
        .filter(|&n_squared| is_odd(n_squared)) // Выбираем нечётные
        .sum(); // Складываем
    println!("Функциональный стиль: {}", sum_of_squared_odd_numbers);
    // >> Функциональный стиль: 5456
}

// 9.4 Расходящиеся функции
/*
Расходящиеся функции никогда не возвращают результат.
Они помечены с помощью !, который является пустым типом.

В отличие от всех других типов, этот не может быть создан, потому что набор всех возможных значений этого типа пуст.
Обратите внимание, что он отличается от типа (), который имеет ровно одно возможное значение.

Хотя это может показаться абстрактным понятием, на самом деле это очень полезно и может пригодится.
Главное преимущество этого типа в том, что его можно привести к любому другому типу и поэтому используется в местах,
    где требуется точный тип, например в ветвях match.
*/

fn foo() -> ! {
    panic!("Этот вызов никогда не вернёт управление.")
}

fn some_fn() {
    ()
}

fn divergent() {
    let a = some_fn();
    println!("Эта функция возращает управление и вы можете увидет эту строку");
    // >> Эта функция возращает управление и вы можете увидет эту строку

    // Произойдёт передача управления в фунцию, но обратно оно уже не вернётся
    // let x = foo();
    // println!("вы никогда не увидете эту строку");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            /*
            Обратите внимание, что возвращаемый тип этого выражения match должен быть u32
                потому что такой тип в переменной "addition" . 
            */
            let addition: u32 = match i % 2 == 1 {
                true => i,
                /*
                С другой стороны выражение "continue" не возвращает
                u32, но это тоже нормально, потому что это тип не возвращающий управление,
                не нарушает требования к типу выражения match. 
                */
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!(
        "Сумма нечётных чисел до 9 (исключая): {}",
        sum_odd_numbers(9)
    );
    // >> Сумма нечётных чисел до 9 (исключая): 16
    /*
    Это также возвращаемый тип функций, которые содержат вечный цикл (например, loop {}), 
        как сетевые серверы или функции, завершающие процесс (например, exit()).
    */
}

pub fn run9() {
    func();
    method();
    circuit();
    circuit_capture();
    input_param_circuit();
    annotated_circuit();
    output_func_circuit();
    output_params_circuit();
    example_iterator_any();
    example_iterator_find();
    func_fvp();
    divergent();
}
