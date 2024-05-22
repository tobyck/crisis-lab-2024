export class RingBuffer<T> {
    array: T[];
    length: number;
    start: number = 0;
    constructor(length: number) {
        this.array = Array(length).fill(null);
        this.length = length;
    }
    get (num: number): T {
        if (num >= this.length || num < 0) {
            throw "Out of bounds";
        }
        return this.array[(num + this.start) % this.length];
    }
    set (num: number, item: T) {
        if (num >= this.length || num < 0) {
            throw "Out of bounds";
        }
        this.array[(num + this.start) % this.length] = item;
    }
    pushpop (item: T): T {
        let old = this.array[this.start];
        this.array[this.start] = item;
        this.start = (this.start + 1) % this.length;
        return old;
    }
    toArray () {
        return this.array.slice(this.start).concat(this.array.slice(0, this.start))
    }
}