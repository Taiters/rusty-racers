import { LocationLayout } from "salesman";
import { getInput } from "./form";

export class Config {
    locationsInput: HTMLInputElement;
    populationInput: HTMLInputElement;
    layoutInput: HTMLInputElement;
    crossoverInput: HTMLInputElement;
    mutationInput: HTMLInputElement;

    cb: ((form: Config) => void) | null = null;

    constructor() {
        this.locationsInput = this.registerInput(getInput("locations"))
        this.populationInput = this.registerInput(getInput("population"));
        this.layoutInput = this.registerInput(getInput("layout"));
        this.crossoverInput = this.registerInput(getInput("crossover"));
        this.mutationInput = this.registerInput(getInput("mutation"));
    }

    registerInput(input: HTMLInputElement): HTMLInputElement {
        input.addEventListener('input', () => this.changed());
        return input;
    }

    locations() {
        return Number.parseInt(this.locationsInput.value);
    }

    population() {
        return Number.parseInt(this.populationInput.value);
    }

    layout() {
        switch (this.layoutInput.value) {
            case "random":
                return LocationLayout.Random;
            case "circle":
                return LocationLayout.Circle;
            default:
                throw Error(`Unexpected layout value: ${this.layoutInput.value}`)
        }
    }

    crossover() {
        return Number.parseFloat(this.crossoverInput.value);
    }

    mutation() {
        return Number.parseFloat(this.mutationInput.value);
    }

    changed() {
        if (this.cb != null) {
            this.cb(this);
        }
    }

    onChange(cb: (form: Config) => void) {
        this.cb = cb;
    }
}
