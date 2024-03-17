import discord
from config import settings
from discord.ext.commands.bot import Bot
from gladoid_bevy import create_world

bot_token = settings.BOT_TOKEN


class Gladoid(Bot):
    async def on_ready(self):
        await self.tree.sync()
        print(f"Logged on as {self.user}!")


intents = discord.Intents.default()
intents.message_content = True

bot = Gladoid(intents=intents, command_prefix="gl!")


@bot.tree.command()
async def play(interaction: discord.Interaction) -> None:
    world = create_world()
    await interaction.response.send_message(
        "Game world successfully created... Beginning the game."
    )

    while True:
        try:
            world.tick()
            if player := world.check_need_action_from():
                await interaction.followup.send(
                    f"Need action from {player.name}... Too slow. "
                    f"I am trying to attack myself or someone, since my maker dont know what he's doing, I'll just randomly attack someone..."
                )
                world.insert_action(action_id=1, player_id=1)
            messages = world.get_game_messages()
            if messages:
                await interaction.followup.send("\n".join(messages))

        except RuntimeError:
            await interaction.followup.send("Game has ended, someone won.")
            break


bot.run(bot_token)
