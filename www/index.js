import init, { Game } from "./build/wasm_bean_counters.js"

const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");
let lastFrame = Date.now();

init().then(() => {
	const game = Game.new();

	canvas.addEventListener("mousemove", (event) => onMove(game, event));

	requestAnimationFrame(() => frame(game));
});

function frame(game)
{
	const delta = Date.now() - lastFrame;
	lastFrame = Date.now();

	game.update(delta);
	game.draw(context);
	requestAnimationFrame(() => frame(game));
}

function onMove(game, event)
{
	let rect = canvas.getBoundingClientRect();
	let x = event.clientX - rect.left;
	let y = event.clientY - rect.top;
	game.mouse_moved(x, y);
}
