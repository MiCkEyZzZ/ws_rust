# Создание многопоточного веб-сервера

## План для создания веб-сервера

* Узнать немного о протоколах TCP и HTTP
* Прослушивать TCP соединения у сокета
* Разобрать небольшое количество HTTP-запросов
* Создать правильный HTTP ответ
* Улучшить пропускную способность нашего сервера с помощью пула потоков
* Добавить больше документации в ThreadPool и его публичные методы
* Добавить тесты для функционала, реализуемого библиотекой
* Заменить вызовы unwrap на более устойчивую обработку ошибок
* Применить ThreadPool для выполнения каких-то других задач, помимо обслуживания веб-запросов
