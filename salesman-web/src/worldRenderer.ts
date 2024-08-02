import { WorldManager } from "./worldManager";


export class WorldRenderer {
    ctx: CanvasRenderingContext2D;
    worldManager: WorldManager;

    constructor(world: WorldManager, ctx: CanvasRenderingContext2D) {
        this.ctx = ctx;
        this.worldManager = world;
    }

    render() {
        this.ctx.canvas.width = 255;
        this.ctx.canvas.height = 255;

        this.ctx.clearRect(-1, -1, this.ctx.canvas.width, this.ctx.canvas.height);

        this.ctx.globalAlpha = 0.5;
        this.ctx.strokeStyle = "blue";
        this.renderGenome(this.worldManager.fittestGenome);

        this.ctx.globalAlpha = 1;
        this.ctx.strokeStyle = "green";
        this.renderGenome(this.worldManager.globalFittestGenome);

        this.ctx.fillStyle = "red";
        for (let i = 0; i < this.worldManager.locations.length; i += 2) {
            const x = this.worldManager.locations[i];
            const y = this.worldManager.locations[i+1];

            this.ctx.beginPath();
            this.ctx.arc(x, y, 2, 0, 2 * Math.PI);
            this.ctx.fill();
        }
    }

    renderGenome(genome: Uint8Array) {
        this.ctx.beginPath()
        this.ctx.moveTo(
            this.worldManager.locations[genome[0] * 2],
            this.worldManager.locations[genome[0] * 2 + 1],
        );

        for (let i = 1; i < genome.length; i++) {
            const x = this.worldManager.locations[genome[i] * 2];
            const y = this.worldManager.locations[genome[i] * 2 + 1];
            this.ctx.lineTo(x, y);
        }

        this.ctx.lineTo(
            this.worldManager.locations[genome[0] * 2],
            this.worldManager.locations[genome[0] * 2 + 1],
        );
        this.ctx.stroke();
    }
}
