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
    width!: number;
    height!: number;

    tickCb: (() => void) | null = null;

    constructor(config: Config, memory: WebAssembly.Memory) {
        this.config = config;
        this.memory = memory;
        this.config.onChange(() => this.updateWorld());
        this.updateWorld();
    }

    updateWorld() {
        const settings = WorldSettings.new(
            this.config.width(),
            this.config.height(),
            this.config.locations(),
            this.config.population(),
            this.config.layout(),
            this.config.crossover(),
            this.config.mutation(),
        );

        this.world = World.new(settings);
        this.generations = 0;
        this.locations = new Uint8Array(
            this.memory.buffer,
            this.world.locations(),
            this.world.location_count() * 2,
        );
        this.bestFitness = 0;
        this.replaceFittest(this.world.fittest());
        this.width = this.config.width();
        this.height = this.config.height();

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