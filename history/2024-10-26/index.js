import init, { Game } from "./build/wasm_bean_counters.js"

const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");
let lastFrame = performance.timeOrigin + performance.now();

init().then(() => {
	const game = Game.new();

	canvas.addEventListener("mousemove", (event) => mouse_event(game, event));
	canvas.addEventListener("mousedown", (event) => mouse_event(game, event));

	requestAnimationFrame(() => frame(game));
});
function frame(game)
{
	let now = (performance.timeOrigin + performance.now());
	let delta = now - lastFrame;
	lastFrame = now;

	game.update(delta);
	game.draw(context);
	requestAnimationFrame(() => frame(game));
}

function mouse_event(game, event)
{
	let rect = canvas.getBoundingClientRect();
	let x = event.clientX - rect.left;
	let y = event.clientY - rect.top;

	switch (event.type)
	{
		case "mousemove": game.mouse_moved(x, y); break;
		case "mousedown": game.mouse_pressed(x, y); break;
	}
}
