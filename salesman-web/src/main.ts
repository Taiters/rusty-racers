import './index.css';

import init from "salesman";
import { Config } from "./config";
import { WorldManager } from "./worldManager";
import { WorldRenderer } from "./worldRenderer";

import playSVG from "./icons/play.svg?raw";
import pauseSVG from "./icons/pause.svg?raw";

const config = new Config();

const tickBtn = <HTMLButtonElement>document.getElementById("tick");
const runBtn = <HTMLButtonElement>document.getElementById("run");
const resetBtn = <HTMLButtonElement>document.getElementById("reset");
const locationsMap = <HTMLCanvasElement>document.getElementById("locations-map");
const ctx = <CanvasRenderingContext2D>locationsMap.getContext("2d");
const globalLowestDistance = <HTMLElement>document.getElementById("global-lowest-distance");
const generationLowestDistance = <HTMLElement>document.getElementById("generation-lowest-distance");
const generationsCounter = <HTMLElement>document.getElementById("generations");

(<HTMLElement>document.getElementById("year")).innerText = `${new Date().getFullYear()}`


init().then((instance) => {
    const worldManager = new WorldManager(config, instance.memory);
    const renderer = new WorldRenderer(worldManager, ctx);

    let running = false;
    let animationFrameRequest: number | null = null;

    worldManager.onTick(() => {
        generationsCounter.innerText = `${worldManager.generations}`;
        generationLowestDistance.innerText = `${(1 / worldManager.fittest.fitness).toFixed(2)}`;
        globalLowestDistance.innerText = `${(1 / worldManager.bestFitness).toFixed(2)}`;
        renderer.render();
    });

    const stopRunning = () => {
        if (animationFrameRequest != null) {
            cancelAnimationFrame(animationFrameRequest);
        }
        tickBtn.disabled = false;
        runBtn.innerHTML = playSVG;
        running = false;
    }

    const startRunning = () => {
        tickBtn.disabled = true;
        running = true;
        function onFrame() {
            worldManager.tick();
            animationFrameRequest = requestAnimationFrame(onFrame);
        }
        animationFrameRequest = requestAnimationFrame(onFrame);
        runBtn.innerHTML = pauseSVG;
    }

    tickBtn.onclick = () => worldManager.tick();
    resetBtn.onclick = () => {
        stopRunning();
        worldManager.updateWorld();
    }
    runBtn.onclick = () => {
        if (running) {
            stopRunning();
        } else {
            startRunning();
        }
    }

    worldManager.updateWorld();
});
