## Running the game server (for now)

>im assuming most of yall dont wanna install the dependencies to run the rewritten game server to your physical computer. therefore, i am using Docker

1. make sure u have docker installed (most of yall are cs/ece so id assume so)
2. Run `docker-compose up --build` inside `game_rs/` subdirectory in `server/`
3. you should get the output. rn the only output is just the test code from `state.rs` which is a rewritten Rust version of `game_state.go` that Princeton wrote, wrapped into Python
<img width="1919" height="1055" alt="image" src="https://github.com/user-attachments/assets/7a7496d5-ee36-4a02-a4e5-a80dd19d4ac7" />
4. continue rewriting the server via Rust and if there are any new ideas let me know. please let me know which parts you are rewriting (**anikoni2010 on discord**) before you do so tho and update periodically ON DISCORD! Thanks!

---------------------------
This repository contains code for an updated version of the Pacbot competition. Contributions are welcome (in the form of pull requests or issues), and please feel free to reach out to ih2422@princeton.edu if you have any questions regarding the game or client implementations.

There are four sub-folders, one for each part of the competition's infrastructure:
* `bot_client`: sample Python code for creating a Pacbot high-level client and algorithm
* `cv_client`: sample Python code for creating a Pacbot CV client, with a tutorial notebook explaining how it works
* `server`: a Go (programming language) implementation of the Pacbot server and game engine
* `web_client`: a Svelte-based (JavaScript web frontend) client to serve as a visualizer and game console
