const data = Array(40);

function* randGenerator(avg, variation, bound) {
    let val = avg;
    while (true) {
        yield 5;
        /*let dv = Math.random() * variation;
        if (Math.random() > (val - avg + bound) / bound / 2) {
            val += dv;
        } else {
            val -= dv
        }*/
    }
}

const baseHeight = 5;

const heightGen = randGenerator(5, 0.1, 3);

const blocks = [];

let addNewBlock = () => {
    blocks.push({
        height: heightGen.next().value,
        x: blockCount / 2,
        vx: 0.1, // 1 block per tick
    });
    //if (blocks.length > blockCount) blocks.shift();
}

const ctx = canvas.getContext('2d');

const blockwidth = 10;
const blockCount = canvas.width / blockwidth;

const slope_width = 20;
const slope_height = 10;

const height_loss = 0.5 / 20;

let get_base_height = (x) => {
    if (x >= blockCount - slope_width) {
        return (slope_height * (x - blockCount + slope_width)) / 2;
    }
    return 0;
}

//addNewBlock();


let render = () => {
    addNewBlock();
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = 'red';
    ctx.globalAlpha = 1;
    ctx.beginPath();
    ctx.moveTo(canvas.width, canvas.height);
    ctx.lineTo(canvas.width - slope_width * blockwidth, canvas.height);
    ctx.lineTo(canvas.width, canvas.height - slope_height * blockwidth);
    ctx.fill();

    let newBlocks = [];

    for (let block of blocks) {
        block.x += block.vx;
        // if now touching slope, split the block

        if (block.x >= blockCount - slope_width) block.vx -= 0.00025;
        if (block.x >= blockCount - slope_width && block.vx > 0) {

            let newBlock = {
                height: block.height - height_loss,
                x: block.x,
                vx: block.vx,
            };
            //block.x = blockCount - slope_width;
            block.height = height_loss;
            block.vx = -block.vx;
            //block.x -= block.vx;
            if (newBlock.height >= 0) newBlocks.push(newBlock);
        }



        if (block.x >= blockCount) {
            block.x = (blockCount - (block.x - blockCount));
            block.vx = -block.vx;
        }

        if (block.x < 0) {
            blocks.splice(blocks.indexOf(block), 1);
        }
    }

    blocks.push(...newBlocks);

    let heights = Array(blockCount).fill(0);
    for (let block of blocks) {
        heights[Math.floor(block.x)] += block.height * (block.x % 1);
        heights[Math.ceil(block.x)] += block.height * (1 - block.x % 1);
    }

    ctx.fillStyle = 'green';
    ctx.beginPath();
    ctx.moveTo(0, canvas.height);
    for (let i = 0; i < blockCount; i++) {
        ctx.lineTo(i * blockwidth, canvas.height - heights[i] / 15 * blockwidth - get_base_height(i));
    }
    ctx.lineTo(canvas.width, canvas.height);
    ctx.stroke();

    ctx.fillStyle = 'dodgerblue';
    ctx.globalAlpha = 0.3;
    for (let block of blocks) {
        ctx.fillRect(block.x * blockwidth, canvas.height - block.height * blockwidth - get_base_height(block.x), 1, block.height * blockwidth);
    }

    ctx.fillStyle = 'black';
    ctx.globalAlpha = 1;
    ctx.font = '20px sans-serif';
    ctx.fillText('Blocks: ' + blocks.length, 100, 100);
}

setInterval(render, 1000 / 20);