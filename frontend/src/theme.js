import { reactive, computed } from 'vue';

let defaultMode = localStorage.getItem('dark')
    ? localStorage.getItem('dark') == 'true'
    : window.matchMedia?.('(prefers-color-scheme: dark)')?.matches;

const LIGHT = {
    textColor: '#122c34',
    backgroundColor: '#f4f4f9',
    graphColor: '#519e8a',
    backgroundColor2: '#f0f0f6',
    statusLightOn: '#00ff00',
    statusLightOff: '#ff0000',
    backgroundColor3: '#e5e5ea',
    graphColor2: '#4ea5d9'
}

const DARK = {
    textColor: '#f4f4f9',
    backgroundColor: '#10262e',
    graphColor: '#abebd2',
    backgroundColor2: '#122c34',
    statusLightOn: '#00ff00',
    statusLightOff: '#ff0000',
    backgroundColor3: '#1a3f4c',
    graphColor2: '#78bce2'
}

export const THEME = reactive({
    defaultMode,
    dark: defaultMode,
    toggleDark: () => {
        localStorage.setItem('dark', THEME.dark = !THEME.dark);
    },
    isMobile: computed(() => window.innerWidth < 900), // TODO: user agent
});

for (let key in LIGHT) {
    THEME[key] = computed(() => THEME.dark ? DARK[key] : LIGHT[key]);
}