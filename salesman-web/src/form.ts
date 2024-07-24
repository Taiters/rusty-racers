export function getInput(id: string): HTMLInputElement {
    const input = <HTMLInputElement>document.getElementById(id);
    const output = document.getElementById(`${id}-value`);
    if (output) {
        output.innerText = input.value;
        input.addEventListener('input', (e) => {
            const target = e.target;
            if (target instanceof HTMLInputElement) {
                output.innerText = target.value;
            }
        });
    }
    return input;
}
