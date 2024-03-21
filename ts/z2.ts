import fs from 'fs';

async function parseLinesFromFile(filePath: string) {
    const content = fs.readFileSync(filePath).toString('utf-8')
    return content.split('\n').filter(notEmpty => notEmpty);
}

type GameData = {
    red: number;
    green: number;
    blue: number;
}

type GameInfo = {
    id: number;
    games: GameData[];
};

function parseData(data: string): GameData {
   const gd = data.split(',').map(x => x.trim())
   .map(x => x.split(' '))
   .reduce((acc, [num, str]) => {
    (acc as any)[str] = +num;
    return acc;
   }, {
    red: 0,
    green: 0,
    blue: 0,
   });

   return gd;
}

function getGame(str: string): GameInfo {
    const [game, rawGames] = str.split(':');
    const id = +game.split(' ').pop()!;
    const games = rawGames.split(';').map(parseData);

    return {
        id,
        games,
    };
}

function isPossible(games: GameData[], limit: GameData = {red: 12, green: 13, blue: 14}) {
    return games.every(game => game.red <= limit.red && game.blue <= limit.blue && game.green <= limit.green);
}

function solvePart1(all: GameInfo[]) {
    const valid = all.filter(gd => isPossible(gd.games));
    return valid.reduce((acc, info) => acc + info.id, 0);
}

function solvePart2(all: GameInfo[]) {
    const max = all.map(x => x.games.reduce((max, gd) => {
      max.blue = Math.max(gd.blue, max.blue);
      max.green = Math.max(gd.green, max.green);
      max.red = Math.max(gd.red, max.red);
      
      return max;
    }));

    return max.reduce((pow, gd) => {
        pow += gd.blue * gd.red * gd.green;
        return pow;
    }, 0);
}

async function main() {
    const data = await parseLinesFromFile('../inputs/z2.txt');
    const all = data.map(getGame);
   

    console.log(`result#1 ${solvePart1(all)}`);
    console.log(`result#2 ${solvePart2(all)}`);
}

main();