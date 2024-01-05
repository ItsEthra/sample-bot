#!/usr/bin/env python

import asyncio
import dotenv
import aiogram
import aiogram.types as types

from typing import cast
from aiogram import Bot, Dispatcher
from aiogram.filters import CommandStart
from aiogram.enums import ParseMode
from aiogram.methods import AnswerInlineQuery

TOKEN = cast(str, dotenv.get_key('.env', 'STORE_BOT_TOKEN'))

dp = Dispatcher()

@dp.callback_query()
async def on_callback(query: types.CallbackQuery):
    await query.answer()

@dp.message(CommandStart())
async def on_message(msg: types.Message):
    app = types.WebAppInfo(url='https://itsethra.github.io/sample-bot')
    kb = types.InlineKeyboardMarkup(inline_keyboard=[
        [
            types.InlineKeyboardButton(text="Открыть сайт", url="https://mirea.ru"),
            types.InlineKeyboardButton(text="Открыть интернет-магазин", web_app=app)
        ]
    ])
    await msg.answer("Выберите сайт или интернет-магазин", reply_markup=kb)

async def main():
    bot = Bot(TOKEN, parse_mode=ParseMode.HTML)
    await dp.start_polling(bot)

if __name__ == '__main__':
    asyncio.run(main())

