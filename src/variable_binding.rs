// Атрибут, который убирает предупреждения компилятора о неиспользуемом коде
#![allow(dead_code)]
#![allow(unused_variables)]

// 4 Связывание переменных

/*
Rust предоставляет безопасность типов с помощью статической типизации.
Тип переменной может быть указан при объявлении связи с переменной.
Тем не менее, в большинстве случаев, компилятор сможет определить тип переменной из контекста, что часто позволяет избавиться от бремени аннотирования кода.
Значения (как и литералы) могут быть привязаны к переменным, используя оператор let.
*/

fn binding_variable() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // Копируем `an_integer` в `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    // >> An integer: 1
    println!("A boolean: {:?}", a_boolean);
    // >> A boolean: true
    println!("Meet the unnit value: {:?}", unit);
    // >> Meet the unnit value: ()

    // Компилятор предупреждает о неиспользуемых переменных; эти предупреждения можно скрыть, поставив знак подчёркивания в начало имени переменной
    let _unused_variable = 3u32;
    let noisy_unused_variable = 2u32;
}

// 4.1 Изменяемость
// По умолчанию связывание переменных является неизменяемым, но с помощью модификатора `mut` изменения можно разрешить.

fn changeability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Перед изменением: {}", mutable_binding);
    // >> Перед изменением: 1

    mutable_binding += 1;

    println!("После изменения: {}", mutable_binding);
    // >> После изменения: 2

    // Ошибка! Переменная не изменяемая
    // _immutable_binding +=1;
}

// 4.2 Область видимости и затенение
/*
Связывание переменных происходит в локальной области видимости — они ограничены существованием внутри блока.
Блок — это набор инструкций, заключённый между фигурными скобками {}.
*/

fn advantage_and_shading_area() {
    // Эта переменная живёт в функции main
    let long_lived_binding = 1;

    // Это блок, он имеет меньшую область видимости, чем функция main
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        // >> inner short: 2
        // При этом в блоке доступна переменная из `main`
        println!("outer long: {}", long_lived_binding);
        // >> outer long: 1
    }
    // Ошибка! `short_lived_binding` нет в этой области видимости
    // println!("outer short: {}", short_living_binding);
    println!("outer long: {}", long_lived_binding);
    // >> outer long: 1

    let shadowed_binding = 1;

    {
        println!("До затенения: {}", shadowed_binding);
        // >> До затенения: 1

        // Эта переменная *затеняет* внешнюю
        let shadowed_binding = "abc";

        println!("Затеннёная во внутреннем блоке: {}", shadowed_binding);
        // >> Затеннёная во внутреннем блоке: abc
    }
    println!("Во внешнем блоке: {}", shadowed_binding);
    // >> Во внешнем блоке: 1

    // Эта привязка *затеняет* предыдущую
    let shadowed_binding = 2;
    println!("Затеннёная во внешнем блоке: {}", shadowed_binding);
    // >> Затеннёная во внешнем блоке: 2
}

// 4.3 Предварительное объявление
/*
Можно сначала объявить связь с переменной, а инициализировать её позже.
Однако такая форма используется редко, так как может привести к использованию неинициализированных переменных.
*/

fn pre_announcement() {
    // Объявляем связь с переменной
    let a_binding;

    {
        let x = 2;
        // Инициализируем связь
        a_binding = x * x;
    }
    println!("Связь a: {}", a_binding);
    // >> Связь a: 4

    let another_binding;

    // Ошибка! Использование неинициализированной связи с переменной
    // println!("другая связь: {}", another_binding);

    another_binding = 1;

    println!("другая связь: {}", another_binding);
    // >> другая связь: 1
    /*
    Компилятор запрещает использование неинициализированных переменных,
        так как это привело бы к неопределённому поведению.
    */
}

// 4.4 Заморозка
/*
Когда данные неизменяемо привязаны к тому же имени, они замораживаются.
Замороженные данные не могут быть изменены до тех пор, пока неизменяемая привязка не выйдет из области видимости:
*/
fn freezing() {
    let mut _mutable_integer = 7i32;
    {
        // Неизменяемое затенение `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Ошибка! `_mutable_integer` заморожена в этой области
        // _mutable_integer = 50;

        // `_mutable_integer` выходит из области видимости
    }
    _mutable_integer = 3;
    println!("_mutable_integer: {}", _mutable_integer);
    // >> _mutable_integer: 3
}
pub fn run4() {
    binding_variable();
    changeability();
    advantage_and_shading_area();
    pre_announcement();
    freezing();
}
