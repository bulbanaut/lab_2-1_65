use std::io::stdin;

/*
Проверять на принадлежность окружности прямую по радиусу
окружность - (x-x0)^2+(y-y0)^2=R^2
прямая - y = kx + b
для отдельной точки (x; y) должно быть верно уравнение x^2 + y^2 = r^2
x^2 + (kx + b)^2 = r^2 замена у
x^2 + (kx)^2 + 2kxb + b^2 - r^2 = 0 раскрытие скобок
d = (2kb)^2 - 4*(k^2+1)*(b^2 - r^2) дискриминант
d = b^2     - 4*   a   *    c
!какая-то беда с обработкой k, 
TODO: починить
*/

fn main() {
    let r: f64 = read_var();
    let k: f64 = read_var();
    let b: f64 = read_var();

    let d = (2.0*k*b).powi(2) - 4.0 * (k.powi(2) + 1.0) * (b.powi(2) - r.powi(2));
    let x1 = (-(2.0*k*b).powi(2) + d.sqrt()) / 2.0 * (k.powi(2) + 1.0);
    let x2 = (-(2.0*k*b).powi(2) - d.sqrt()) / 2.0 * (k.powi(2) + 1.0);
    let y1 = k*x1 + b;
    let y2 = k*x2 + b;

    if d < 0.0 {
        println!("Общих точек нет");
    } else if d == 0.0{
        println!("Прямая - касательна");
        println!("Точка: {}, {}", x1, y1 );

        quarter_match(x1, y1, 1);
    } else {
        println!("Точка 1: {}, {}", x1, y1);
        println!("Точка 2: {}, {}", x2, y2);

        quarter_match(x1, y1, 1);
    
        quarter_match(x2, y2, 2);
    }
}

fn quarter_match(x:f64, y:f64, n:u8) {
    if x > 0.0 && y > 0.0 {
        println!("Точка {n} находится в четрверти 1");
    } else if x > 0.0 && y < 0.0 {
        println!("Точка {n} находится в четрверти 2");
    } else if x < 0.0 && y < 0.0 {
        println!("Точка {n} находится в четрверти 3");
    } else if x < 0.0 && y < 0.0 {
        println!("Точка {n} находится в четрверти 4");
    } else {
        println!("Точка {n} находится на границе четвертей");
    }
}

fn read_var() -> f64 {
    loop {
        let mut x =String::new();
        stdin()
            .read_line(&mut x)
            .expect("Ошибка чтения ввода"); //считывание ввода и запись его в переменную X (с точки зрения компилятора это одна строка)

        let x: f64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Переменная должна быть равна числу");
                continue;
            },
        }; //преобразование ввода со string в float-point integer с перезапуском loop в случае ошибки
        break x;
    }
}