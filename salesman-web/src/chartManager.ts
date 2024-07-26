import { Chart } from "chart.js/auto";

export class ChartManager {
    chart: Chart;
    container: HTMLElement;

    constructor(canvas: HTMLCanvasElement, container: HTMLElement, toggleBtn: HTMLButtonElement) {
        this.container = container;
        this.chart = new Chart<"line", number[], number>(
            canvas,
            {
                type: "line",
                data: {
                    labels: [],
                    datasets: [
                        {
                            label: 'numbers',
                            data: [],
                        }
                    ]
                },
                options: {
                    scales: {
                        y: {
                            beginAtZero: true
                        }
                    }
                }
            }
        );

        toggleBtn.addEventListener('click', () => {
            this.container.hidden = !this.container.hidden;
            if (!this.container.hidden) {
                this.update();
                toggleBtn.innerText = "Hide Chart";
            } else {
                toggleBtn.innerText = "Show Chart";
            }
        });
    }

    reset() {
        this.chart.data.labels = [];
        this.chart.data.datasets[0].data = [];
        this.update();
    }

    append(generation: number, value: number) {
        this.chart.data.labels?.push(generation)
        this.chart.data.datasets[0].data?.push(value);
        this.update();
    }

    update() {
        if (!this.container.hidden) {
            this.chart.update('none');
        }
    }
}
