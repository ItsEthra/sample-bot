#!/usr/bin/env python

import asyncio
from aiogram.types.input_file import BufferedInputFile
import dotenv
import aiogram
import redis.asyncio as redis
import aiogram.types as types

from typing import cast
from aiogram import Bot, Dispatcher
from aiogram.filters import CommandStart
from aiogram.enums import ParseMode
from aiogram.methods import AnswerInlineQuery

TOKEN = cast(str, dotenv.get_key('.env', 'COMICS_BOT_TOKEN'))

dp = Dispatcher()
db = redis.Redis(host='localhost', port=6379, decode_responses=True)

@dp.callback_query()
async def on_callback(query: types.CallbackQuery):
    comic, i = cast(str, query.data).split(':')

    num = int(i)
    max_len = int(await db.hget("comics", comic))

    kbd = [[],[]]

    if num != max_len:
        kbd[0].append(types.InlineKeyboardButton(text="Вперёд", callback_data=f"{comic}:{num + 1}"))

    if num != 1:
        kbd[0].append(types.InlineKeyboardButton(text="Назад", callback_data=f"{comic}:{num - 1}"))
        kbd[1].append(types.InlineKeyboardButton(text="В начало", callback_data=f"{comic}:1"))
    
    try:
        with open(f"comics/{comic}/{i}.png", 'rb') as f:
            await query.message.edit_media(media=types.InputMediaPhoto(media=BufferedInputFile(f.read(), "comic.png")), reply_markup=types.InlineKeyboardMarkup(inline_keyboard=kbd))
        await query.answer()
    except Exception as e:
        print(e)
        await query.answer("Нет такого комикса", show_alert=True)


@dp.message(CommandStart())
async def on_message(msg: types.Message):
    kbd = [];
    all = await db.hgetall("comics")
    for comic in all.keys():
        kbd.append([types.KeyboardButton(text=comic)])
    
    await msg.answer("Выберите комикс путем нажатия на кнопку", reply_markup=types.ReplyKeyboardMarkup(keyboard=kbd))

@dp.message()
async def on_comic(msg: types.Message):
    kbd = [[]]

    max_len = int(await db.hget("comics", comic))
    if max_len != 1:
        kbd[0].append(types.InlineKeyboardButton(text='Вперёд', callback_data=f'{msg.text}:2'))        

    try:
        with open(f"comics/{msg.text}/1.png", 'rb') as f:
            await msg.answer_photo(photo=types.BufferedInputFile(f.read(), "comic.png"), reply_markup=types.InlineKeyboardMarkup(inline_keyboard=kbd))
    except:
        await msg.answer("Нет такого комикса")

async def main():
    bot = Bot(TOKEN, parse_mode=ParseMode.MARKDOWN_V2)
    await dp.start_polling(bot)

if __name__ == '__main__':
    asyncio.run(main())


