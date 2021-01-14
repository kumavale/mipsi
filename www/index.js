import * as wasm from "hello-wasm-pack";

console.log("start loading wasm");
const mipsi = import('../pkg').catch(console.error);

Promise.all([mipsi]).then(async function ([
    { draw_canvas, wasm_run }
]) {
    console.log("finished loading wasm");
    var canvas_w = 64;
    var canvas_h = 64;
    var unit_w = 1;
    var unit_h = 1;
    reset_canvas(canvas_w, canvas_h, unit_w, unit_h);
    const run_button = document.getElementById('run_button');
    run_button.addEventListener('click', () => {
        const src = document.getElementById('src');
        if (src.value == "") {
            alert("Please fill out textarea.");
        } else {
            wasm_run();
        }
    });
});

function reset_canvas(canvas_w, canvas_h, unit_w, unit_h) {
    var canvas = document.getElementById('canvas_wasm');
    canvas.width  = canvas_w * unit_w;
    canvas.height = canvas_h * unit_h;
    var ctx = canvas.getContext("2d");
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}

//setInterval(function() {
//    console.log("hoge");
//}, 1000);
