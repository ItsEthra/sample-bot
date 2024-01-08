Для запуска необходимо создать `.env` файл с токеном бота `COMICS_BOT_TOKEN=<token>` или просто откройте [@sample_comics_bot](https://t.me/sample_comics_bot) в телеграме(REST API не доступно).

Для работы нужен redis: `docker run --name comics-bot -p 6379:6379 redis`. В репозитории лежит папка `comics` c пятью комиксами `xkcd`, что бы пропустить добавление через REST можно сделать `redis-cli hset comics xkcd 5`

REST API написано на rust + axum так как в тз про язык REST ничего сказано не было `¯\_(ツ)_/¯`.

Доступные эндпоинты:
* GET `/comic/:comic`: Принимает body png file, `:comic` это название комикса который отображается в тг.
По факту просто добавляет новую картинку к комиксу.
`curl --data-binary "@image.png" http://localhost:8080/comic/foo`

* DELETE `/comic/:comic/:num`: `:comic` название комикса из тг, `:num` номер картинки в комиксе.
Просто удаляет картинку из комикса.
`curl http://localhost:8080/comic/foo/3 -X DELETE`


