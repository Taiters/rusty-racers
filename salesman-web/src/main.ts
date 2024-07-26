import init from "salesman";
import { Config } from "./config";
import { WorldManager } from "./worldManager";
import { WorldRenderer } from "./worldRenderer";

const config = new Config();
const tickBtn = <HTMLButtonElement>document.getElementById("tick");
const runBtn = <HTMLButtonElement>document.getElementById("run");
const locationsMap = <HTMLCanvasElement>document.getElementById("locations-map");
const ctx = <CanvasRenderingContext2D>locationsMap.getContext("2d");
const globalLowestDistance = <HTMLElement>document.getElementById("global-lowest-distance");
const generationLowestDistance = <HTMLElement>document.getElementById("generation-lowest-distance");
const generationsCounter = <HTMLElement>document.getElementById("generations");


init().then((instance) => {
    const worldManager = new WorldManager(config, instance.memory);
    const renderer = new WorldRenderer(worldManager, ctx);

    worldManager.onTick(() => {
        generationsCounter.innerText = `${worldManager.generations}`;
        generationLowestDistance.innerText = `${1 / worldManager.fittest.fitness}`;
        globalLowestDistance.innerText = `${1 / worldManager.bestFitness}`;

        renderer.render();
    });

    tickBtn.onclick = () => worldManager.tick();

    let running = false;
    let animationFrameRequest: number | null = null;
    runBtn.onclick = () => {
        if (running) {
            if (animationFrameRequest != null) {
                cancelAnimationFrame(animationFrameRequest);
            }
            tickBtn.disabled = false;
            runBtn.innerText = "Start";
            running = false;
        } else {
            tickBtn.disabled = true;
            running = true;
            function onFrame() {
                worldManager.tick();
                animationFrameRequest = requestAnimationFrame(onFrame);
            }
            animationFrameRequest = requestAnimationFrame(onFrame);
            runBtn.innerText = "Stop";
        }
    }

    renderer.render();
});
