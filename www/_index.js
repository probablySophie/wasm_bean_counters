
// TODO: figure out how to auto update this to our package name
const PKG_NAME = "wasm_bean_counters"
const PKG_PATH = "./wasm_bean_counters_bg.wasm"


// TODO: Also auto update this?
// 		 	Or at least figure out how to import *
import * as wasm from './wasm_bean_counters.js';

// TODO: Use this style instead of the above
// import * as wasm from "hello-wasm-pack";

//localStorage.setItem();

// making global variables
let player = {};
let world = {};

// This has to load before anything else can work properly
// So it can't live in the base.js file
const canvas = document.getElementById("canvas");
const context = canvas.getContext('2d');


let mouse = {
	x: 0,
	y: 0,
}


canvas.addEventListener('mousemove', (e)=> 
{
	let rect = canvas.getBoundingClientRect();
	mouse.x = e.clientX - rect.left;
	mouse.y = e.clientY - rect.top;
	
}, false);




const renderLoop = () => {
	context.clearRect(0, 0, canvas.width, canvas.height);

	//context.fillText(`${player.get_x()}, ${player.get_y()}`, 5, 15);
	
	requestAnimationFrame(renderLoop);
}


async function run()
{	
	
	// TODO: This should also be 
	await wasm.default(PKG_PATH);

	console.info(wasm);

	wasm.start();
	//wasm.start();

	
	//player = Player.new();
	//world = World.new();

	// Render the first frame - the render loop renders all
	// subsequent frames	
	//requestAnimationFrame(renderLoop);

	// make the function available to the browser
}



run();
