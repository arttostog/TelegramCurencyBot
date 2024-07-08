use teloxide::{macros::BotCommands, repls::CommandReplExt, requests::{Requester, ResponseResult}, types::Message, utils::command::BotCommands as _, Bot};
mod currency;

#[tokio::main]
async fn main() {
    let telegram_bot: Bot = Bot::from_env();
    Command::repl(telegram_bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
enum Command {
    #[command(description = "Стартовое сообщение.")]
    Start,

    #[command(description = "Показывает все команды.")]
    Help,

    #[command(description = "Показывает курс рубля к доллару.")]
    CheckCurency
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()>{
    match cmd {
        Command::Start => bot.send_message(msg.chat.id, "Привет! Введи `/help` для получения списка команд.").await?,
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::CheckCurency => {
            bot.send_message(msg.chat.id, format!("Текущий курс рубля к доллару: {}", currency::get_value().await)).await?
        }
    };

    Ok(())
}