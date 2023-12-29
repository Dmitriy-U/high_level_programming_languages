// Анонимные функции (лямбды) "closure" замыкания
// |x| x + 2
// first class citizen
// 1 -  литералы,
// 2 - типы данных,
// 3 - можно передать в виде параметра,
// 4 - можно вернуть результат
// |[<параметры>]| [-> <тип результата>] <выражение>
// |x:i32,y:i32| -> i32 {x + y}
// Fn(i32) -> i32

// Трейты
// 1. FnOnce функция вызываемая хотя бы 1 раз (реализует любое замыкание)
// 2. FnMut замыкая захватывающее контекст и изменяет его
// 3. Fn ничего не захватывает, либо захватывает неизменяемые ссылки (Только этот трейт можно вызвать параллельно)

fn bar() {
    let x = 5;
    fn foo() -> () {
        // println!("{}", x); // Ошибка: нельзя брать из другой области видимости
    }
}

fn foo() {
    let mut x = vec![1, 2, 3];
    let mut f = || {
        x.push(4);
        println!("{:?}", x.len());
    };
    // println!("{:?}", x);
    // Зона где x ещё не изменён, но захвачен
    f();
    // Зона где x изменён
    println!("{:?}", x);
}

fn foo_1() {
    let mut x = vec![1, 2, 3];
    let mut f = move || { // move передача владения
        println!("{:?}", x.len());
    };
    f();
    // println!("{:?}", x.len()); // Ошибка:
}

fn foo_2() {
    let mut x: Vec<String> = Vec::new();
    let s1 = String::from("abc");
    let f = || { x.push(s1); };
    f();
    // f(); // Ошибка: s1 уже не существует
}

fn main() {
    foo();
    foo_1();
    foo_2();
}
