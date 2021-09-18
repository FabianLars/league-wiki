# mw-toolbox

some tools to interact with http://leagueoflegends.fandom.com/de/wiki/ needed by the maintenance bot.

uses limited editing rate of ~1-2 edits per second according to fandoms rules

Interaction with Riot's API only with "riot-api" feature flag (just "update rotation" for now) - enabled by default

-   CLI usage (development):
    -   run via "cargo run \<command\>"
        -   these commands need FANDOM_BOT_NAME, FANDOM_BOT_PASSWORD and RIOT_API_KEY (if needed) environment variables.
            -   Fandom login credentials can be alternatively provided via:
                -   "cargo run [--loginname \<name\>|-n \<name\>] and [--loginpassword \<pw\>|-p \<pw\>]"
            -   The login creds needs to be generated via Special:BotPasswords

# Important!

This project isn't endorsed by Riot Games and doesn't reflect the views or opinions of Riot Games or anyone officially involved in producing or managing League of Legends.
League of Legends and Riot Games are trademarks or registered trademarks of Riot Games, Inc. League of Legends © Riot Games, Inc.

Same for Fandom and MediaWiki btw...
