import { WorldManager } from "./worldManager";

const UI_SCALE = 5;
const PADDING = 20;

const convert = (value: number): number => value * UI_SCALE + PADDING;

export class WorldRenderer {
    ctx: CanvasRenderingContext2D;
    worldManager: WorldManager;

    constructor(world: WorldManager, ctx: CanvasRenderingContext2D) {
        this.ctx = ctx;
        this.ctx.canvas.width = 255 * UI_SCALE + (PADDING * 2);
        this.ctx.canvas.height = 255 * UI_SCALE + (PADDING * 2);
        this.worldManager = world;
    }

    render() {
        this.ctx.clearRect(-1, -1, this.ctx.canvas.width, this.ctx.canvas.height);

        this.ctx.globalAlpha = 0.5;
        this.ctx.strokeStyle = "#53c0f3";
        this.ctx.lineWidth = 3;
        this.renderGenome(this.worldManager.fittestGenome);

        this.ctx.globalAlpha = 1;
        this.ctx.strokeStyle = "#71ead2";
        this.ctx.lineWidth = 5;
        this.renderGenome(this.worldManager.globalFittestGenome);

        for (let i = 0; i < this.worldManager.locations.length; i += 2) {
            const x = convert(this.worldManager.locations[i]);
            const y = convert(this.worldManager.locations[i+1]);

            this.ctx.fillStyle = "#71ead2";
            this.ctx.beginPath();
            this.ctx.arc(x, y, 15, 0, 2 * Math.PI);
            this.ctx.fill();

            this.ctx.fillStyle = "#e779c1";
            this.ctx.beginPath();
            this.ctx.arc(x, y, 10, 0, 2 * Math.PI);
            this.ctx.fill();
        }
    }

    renderGenome(genome: Uint8Array) {
        this.ctx.beginPath()
        this.ctx.moveTo(
            convert(this.worldManager.locations[genome[0] * 2]),
            convert(this.worldManager.locations[genome[0] * 2 + 1]),
        );

        for (let i = 1; i < genome.length; i++) {
            const x = convert(this.worldManager.locations[genome[i] * 2]);
            const y = convert(this.worldManager.locations[genome[i] * 2 + 1]);
            this.ctx.lineTo(x, y);
        }

        this.ctx.lineTo(
            convert(this.worldManager.locations[genome[0] * 2]),
            convert(this.worldManager.locations[genome[0] * 2 + 1]),
        );
        this.ctx.stroke();
    }
}
