import fs from 'fs';

async function parseLinesFromFile(filePath: string) {
    const content = fs.readFileSync(filePath).toString('utf-8')
    return content.split('\n').filter(notEmpty => notEmpty);
}

function changeStrToNumStr(str: string): string {
        var all = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine'];
        let x = str.slice(0);
        for (var i=0; i<x.length; i++) {
            const match = all.find(u => x.indexOf(u, i) === i);
            if (match) {
                x = [x.slice(0, i), all.indexOf(match) + 1, x.slice(i + match.length)].join('');
                i = 0;
            }
        }
        return x;
}

function changeStrToNumArray(str: string): number[] {
    const stack = [];
    var all = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine'];
    for (let i = 0; i < str.length; i++) {
        if (str[i] >= '0' && str[i] <= '9') {
            stack.push(str[i]);
        } else {
            const match = all.find(numStr => str.indexOf(numStr, i) === i);
            if (match) {
                stack.push(all.indexOf(match) + 1);
            }
        }
    }
    return stack.map(Number);
}

function toNumArray(str: string): number[] {
    return str.split('').reduce((acc, char) => {
        if (char >= '1' && char <= '9') {
            acc.push(Number(char));
        }
        return acc;
    }, [] as number[]);
}

function getFirstAndLast(nums: number[]): [number, number] {
    return [nums[0], nums.slice(0).pop()!];
}

async function main() {
    const data = await parseLinesFromFile('../inputs/z1.txt');
    const solution2 = data
        //.map(changeStrToNumStr)
        //.map(toNumArray)
        .map(changeStrToNumArray)
        .map(getFirstAndLast)
        .map(([a,b]) => a * 10 + b)
        .reduce((a, b) => a + b, 0);

    console.log(solution2);
}

for (let i = 0; i < 10000; i++) {
    main();
}
