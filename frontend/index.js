import { init, stop } from './emulator/gameboy_emulator';

init();

setTimeout(() => {
    stop();
}, 5000)