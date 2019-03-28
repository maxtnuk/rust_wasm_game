# rust wasm_games

this project is for test_wasm_games 
(running in rust,webgl2)

To Do

- [ ] develope several games
- [x] discuss which frontend platform is better( Yew, Seed ) -> seed
- [ ] consider cargo-web ( without npm run )
- [x] for fast developing make MAKE file: complete (cargo make)
- [ ] develope frontend with seed

## How To use

first of all you need cargo-make

    $ cargo install cargo-make
    
and also need [NPM](https://www.npmjs.com/get-npm) for running web_client

then run this for build under root folder

    $ cargo make build 
    
if you want to run server with npm, run this

    $ cargo make server 

