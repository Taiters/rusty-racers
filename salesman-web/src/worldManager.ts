import { Genome, World, WorldSettings } from "salesman";
import { Config } from "./config";


export class WorldManager {
    config: Config;
    memory: WebAssembly.Memory;

    world!: World;
    fittest!: Genome;
    fittestGenome!: Uint8Array;
    globalFittest!: Genome;
    globalFittestGenome!: Uint8Array;
    locations!: Uint8Array;
    generations: number = 0;
    bestFitness: number = 0;

    tickCb: (() => void) | null = null;

    constructor(config: Config, memory: WebAssembly.Memory) {
        this.config = config;
        this.memory = memory;
        this.config.onChange(() => this.updateWorld());
    }

    updateWorld() {
        const settings = WorldSettings.new(
            255,
            255,
            this.config.locations(),
            this.config.population(),
            this.config.layout(),
            this.config.crossover(),
            this.config.mutation(),
        );

        this.world = World.new(settings);
        this.generations = 0;
        this.locations = this.world.locations();
        this.bestFitness = 0;
        this.replaceFittest(this.world.fittest());

        settings.free();

        if (this.tickCb) {
            this.tickCb();
        }
    }

    onTick(cb: () => void) {
        this.tickCb = cb;
    }

    replaceFittest(fittest: Genome) {
        this.fittest?.free()
        this.fittest = fittest;
        this.fittestGenome = new Uint8Array(
            this.memory.buffer,
            fittest.data,
            this.locations.length / 2,
        );

        if (fittest.fitness > this.bestFitness) {
            this.bestFitness = fittest.fitness;
            this.globalFittestGenome = new Uint8Array(this.fittestGenome);
        }
    }

    tick() {
        this.world.tick();
        this.generations += 1;
        this.replaceFittest(this.world.fittest());

        if (this.tickCb) {
            this.tickCb();
        }
    }
}