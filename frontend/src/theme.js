import { reactive, computed } from 'vue';
export const THEME = reactive({
    alertActive: false,
    dark: true,
    borderColor: computed(() => THEME.alertActive ? 
        alertGradient[Math.floor(THEME.timeStamp / 50) % 32]
        : 'rgb(180, 190, 254)'
    ),
    backgroundColor: computed(() => THEME.alertActive ? 
        alertBackgroundGradient[Math.floor(THEME.timeStamp / 50) % 32]
        : 'rgb(24, 24, 37)'
    ),
    headerColor1: '#94e2d5',
    headerColor2: '#89b4fa',
    textColor: '#cdd6f4',
    alertColor: 'green',
    timeStamp: Date.now(),
})

setInterval(() => {
    THEME.timeStamp = Date.now();
    //console.log(THEME.borderColor.value, alertGradient)
}, 50);



const alertColor1 = [242, 205, 205];
const alertColor2 = [255, 0, 0];

const lerp = (a, b, t) => Math.round(a + (b - a) * t);

const lerpRGB = (a, b, t) => {
    return `rgb(${[
        lerp(a[0], b[0], t),
        lerp(a[1], b[1], t),
        lerp(a[2], b[2], t),
    ]})`;
}

const createRGBGradient = (a, b, count = 16) => {
    let gradient = [];
    for (let i = 0; i < count; i++) {
        gradient.push(lerpRGB(a, b, i / (count-1)));
    }
    return gradient.concat([...gradient].reverse());
}

const alertGradient = createRGBGradient(alertColor1, alertColor2, 16);

let alertBackgroundGradient = createRGBGradient([24, 24, 37], [60, 24, 37], 16);
alertBackgroundGradient = alertBackgroundGradient.slice(10).concat(alertBackgroundGradient.slice(0, 10));