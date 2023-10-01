# Altura's Fastidious Planner Discord Bot
A bot created to conduct matters of RPG scheduling.

## Available Commands
- `/ping`: Can be used to prompt a response in the discord server,
ensures bot is connected correctly.

## Frontend-Only Commands (needs backend implementation)
- `/enroll_pc ${PLAYER_NAME}`: Enrolls someone as player character.
- `/enroll_gm ${PLAYER_NAME}`: Enrolls someone as game master.

## Yet-To-Be-Implemented Commands
- `/prompt_session MM-dd HH:mm`: Prompts players in #general chat for availability on given day.
- `/cancel_session MM-dd HH:mm`: Notifies players in #general chat that there will be no session.
- `/list_party`: Lists all players and their role in the server.
- `/remove_player`: Removes someone from server party.
- `/help`: describes the available bot commands.

## How to Run
1. Get a discord bot api token (not going to elaborate on how to do this)
2. Set api token environment variable in IDE
3. Run it
4. 💰💰💰

# TODO problems must haves
- verify user_id is present is server to prevent accidental subscription
  - Bot must take username, check if its in server, and convert to static discord ID
  - Bot should be able to look up username from discord ID
- get emoticon reactions to prompt message from bot
  - how are we recognizing that the response is to the bots message, and that it is the poll message
- query campaign metadata
- set message frequency
- set threshold for cancelling session
- set notification channel 

# TODO problems nice to have
- for fun, find dnd memes that can be posted on success or failure (we shouldn't do this, but hey its funny)
- establish convenient way of defining campaign. Should it be one command?
  - if we want to support multiple campaigns, we need to be able to create a campaign, then reference it in commands
- set ability to override next session without changing campaign definition



## Joel Section
- Pull up future sessions, call early votes and overwrites
- Let players opt into text notifications if they don't check their discord