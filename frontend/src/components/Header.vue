<template>
    <header>
        <span class = "emoji">&#128680 </span>
        <span class = "textTsunami">A TSUNAMI IS HAPPENING</span>
        <span class= "textNormal">Crisis Labs 2024 Dashboard</span>
        <span class = "emoji"> &#128680</span>
        </header>
</template>

<style scoped>
header>span.textNormal {
    height: 50px;
    font-size: 40px;
    background: -webkit-linear-gradient(315deg, var(--headerColor1), var(--headerColor2));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    display: var(--tsunamiNotHappening);
}

header {
    font-size: 40px;
    text-align: center;
    border-radius: 25px;
    border: 2px solid var(--borderColor);
    margin-left: 5vw;
    margin-right: 5vw;
    padding-top: 1vw;
    padding-bottom: 1vw;
}

@media screen and (min-width: 2400px) {
    header>span {
        font-size: 120px;
    }
}

header> span.emoji {
    display: var(--tsunamiHappening);
}
header> span.textTsunami {
    display: var(--tsunamiHappening);
}
</style>