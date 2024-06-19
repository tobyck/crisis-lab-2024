import { reactive, computed } from 'vue';

// yay dark mode autodetection
let defaultMode = localStorage.getItem('dark')
    ? localStorage.getItem('dark') == 'true'
    : window.matchMedia?.('(prefers-color-scheme: dark)')?.matches;

const LIGHT = {
    textColor: '#122c34',
    backgroundColor: '#e6eaf5',
    graphColor: '#78ccee',
    backgroundColor2: '#f0f1ff',
    statusLightOn: '#44cc22',
    statusLightOff: '#ff007c',
    backgroundColor3: '#E0E3EC',
    graphColor2: '#6a92e7'
}

const DARK = {
    textColor: '#e6eaf8',
    backgroundColor: '#1f2335',
    graphColor: '#89ddff',
    backgroundColor2: '#292e42',
    statusLightOn: '#77ff55',
    statusLightOff: '#ff007c',
    backgroundColor3: '#1C1F2D',
    graphColor2: '#7aa2f7'
}

// Basically this is a magic object that contains either light or dark mode colours depending on the current theme

export const THEME = reactive({
    defaultMode,
    dark: defaultMode,
    toggleDark: () => {
        localStorage.setItem('dark', THEME.dark = !THEME.dark);
    },
    isMobile: computed(() => window.innerWidth < 900), // TODO: stronger check
    alertActive: false,
});

for (let key in LIGHT) {
    THEME[key] = computed(() => THEME.dark ? DARK[key] : LIGHT[key]);
}