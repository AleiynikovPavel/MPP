# MPP
### О проекте
Реализована библиотека на Rust, которую можно вызывать из Java кода через JNI. Библиотека предоставляет 3 метода. Все они принимают массив целых чисел, выбрают числа, которе являются идеальным квадратом и возвращают их сумму.
- `int sumPerfectSquare(int [] arr)` - метод выполняет операцию в один поток
- `int sumPerfectSquarePar(int [] arr)` - метод выполняет операцию в несколько потоков (Используется библиотека [Rayon](https://github.com/rayon-rs/rayon))
- `int sumPerfectSquarePar(int [] arr, int thread_count)` - метод использует thread_count потоков для выполнения операции (Используется библиотека [Rayon](https://github.com/rayon-rs/rayon))

### Сборка
#### Библиотека (Rust)
- `$ cd rust_lib` - переходим в каталог с проектом библиотеки
- `$ cargo build --release` - сборка библиотеки
- `$ cp target/release/liblibmpp.so ../java/` - копируем собранную библиотеку в java проект
#### Вызывающий код (Java)
- `$ cd java/src` - переходим в каталог java проекта
- `$ javac com/company/RustJni.java` - компилируем Java класс RustJni.java
- `$ java -Djava.library.path=../ com.company.RustJni` - запуск java проекта
### Вывод программы
```
sumPerfectSquare time: 290 ns
14
sumPerfectSquarePar time: 558243 ns
14
sumPerfectSquarePar with pool size: 2 time: 37310 ns
14
```
- При вызове каждого метода библиотека выводит в консоль время в ns затраченное на выполнение операции (время потраченное на прибразование типов не включается в этот промежуток)
