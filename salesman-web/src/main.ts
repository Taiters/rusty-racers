import './index.css';

import init from "salesman";
import { Config } from "./config";
import { WorldManager } from "./worldManager";
import { WorldRenderer } from "./worldRenderer";

import playSVG from "./icons/play.svg?raw";
import pauseSVG from "./icons/pause.svg?raw";
import posthog from 'posthog-js';

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

    const getEventProperties = () => ({
        generation: worldManager.generations,
        locations: config.locations(),
        layout: config.layout(),
        population: config.population(),
        crossoverRate: config.crossover(),
        mutationRate: config.mutation(),
    })

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

    tickBtn.onclick = () => {
        worldManager.tick();
        posthog.capture("tick_pressed", getEventProperties());
    }
    resetBtn.onclick = () => {
        stopRunning();
        worldManager.updateWorld();
        posthog.capture("reset_pressed", getEventProperties());
    }
    runBtn.onclick = () => {
        if (running) {
            stopRunning();
            posthog.capture("pause_pressed", getEventProperties());
        } else {
            startRunning();
            posthog.capture("run_pressed", getEventProperties());
        }
    }

    worldManager.updateWorld();
});

posthog.init('phc_8nwD1m0sIgnFFkfe3CSBWzawKIJnNyzCQeIoKKdZUT8',
    {
        api_host: 'https://eu.i.posthog.com',
        person_profiles: 'identified_only' // or 'always' to create profiles for anonymous users as well
    }
)