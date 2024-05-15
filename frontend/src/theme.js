import { reactive, computed } from 'vue';

let defaultMode = localStorage.getItem('dark') ? localStorage.getItem('dark') == 'true' : window.matchMedia?.('(prefers-color-scheme: dark)')?.matches
export const THEME = reactive({
    alertActive: false,
    defaultMode,
    dark: defaultMode,
    borderColor: computed(() => THEME.alertActive ? 
        alertGradient[Math.floor(THEME.timeStamp / 50) % 32]
        : THEME.dark ? 'rgb(180, 190, 254)' : 'rgb(114, 135, 253)'
    ),
    backgroundColor: computed(() => THEME.alertActive ? 
        alertBackgroundGradient[Math.floor(THEME.timeStamp / 50) % 32]
        : THEME.dark ? 'rgb(24, 24, 37)' : 'rgb(230, 233, 239)'
    ),
    headerColor1: computed(() => THEME.dark ? 'rgb(148, 226, 213)' : 'rgb(23, 146, 153)'),
    headerColor2: computed(() => THEME.dark ? 'rgb(137, 180, 250)' : 'rgb(30, 102, 245)'),
    textColor: computed(() => THEME.dark ? '#cdd6f4' : 'rgb(76, 79, 105)'),
    timeStamp: Date.now(),
    gridColor: computed(() => THEME.dark ? 'rgb(49, 50, 68)' : 'rgb(204, 208, 218)'),
    graphColor1: computed(() => THEME.dark ? 'rgb(137, 220, 235)' : 'rgb(4, 165, 229)'),
    graphColor2: computed(() => THEME.dark ? 'rgb(137, 180, 250)' : 'rgb(30, 102, 245)'),
    lighterBackground: computed(() => THEME.dark ? 'rgb(49, 50, 68)' : 'rgb(204, 208, 218)')
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