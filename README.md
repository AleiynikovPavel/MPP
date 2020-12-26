# MPP
### О проекте
Реализована библиотека на Rust, которую можно вызывать из Java кода через JNI. Библиотека предоставляет 3 метода. Все они принимают массив целых чисел, и возвращают их сумму.
- `int sumPerfectSquare(int [] arr)` - метод выполняет операцию в один поток
- `int sumPerfectSquarePar(int [] arr)` - метод выполняет операцию в несколько потоков (Используется библиотека [Rayon](https://github.com/rayon-rs/rayon))
- `int sumPerfectSquarePar(int [] arr, int thread_count)` - метод использует thread_count потоков для выполнения операции (Используется библиотека [Rayon](https://github.com/rayon-rs/rayon))

### Сборка вручную
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
sumPerfectSquare time: 429904 ns
1000000
sumPerfectSquarePar time: 619414 ns
1000000
sumPerfectSquarePar with pool size: 2 time: 313017 ns
1000000
```
- При вызове каждого метода библиотека выводит в консоль время в ns затраченное на выполнение операции (время потраченное на прибразование типов не включается в этот промежуток)

## Скрипты
### Сборка с векторизацией
- `$ bash buildVec.sh` - скрипт собирёт библиотеку и Java программу и запустит её
### Сборка без векторизации
- `$ bash build.sh` - скрипт собирёт библиотеку и Java программу и запустит её
### Запуск Java программы
- `$ bash run.sh`


## Замер скорости
Для оценки эффективности оптимизаций создан Python скрипт (Python 3.x), который запускает N раз Java программу, парсит её вывод и отдать среднее время выполнения вычислений для каждого метода библиотеки
### Запуск
- `$ python3 test_avg.py`
### Вывод скрипта
```
avg no par     439138.501
avg MAX thread 752873.532
avg 2 thread   343105.29
```
