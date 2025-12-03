import fs = require('fs');

function loadFile(): string[] {
    const content = fs.readFileSync('input.txt', 'utf-8');   
    return content.split(',');
}

function checkRange(idRange: string): number[] {
    const ids = idRange.split("-");
    if (ids.length != 2) {
        throw new Error("unexpected id length")
    }
    const start = +ids[0]!;
    const end = +ids[1]!;
    let invalidIDs: number[] = [];

    for (let i = start; i <= end; i++) {
        const strID = i.toString();
        const firstHalf = strID.slice(0, strID.length/2);
        const lastHalf = strID.slice(strID.length/2, strID.length);
        // check for part 1, if it doesn't work then we'll iterate through the word
        if (firstHalf === lastHalf) {
            invalidIDs.push(i);
        } else if (checkForRepeatedDigits(strID)) {
            invalidIDs.push(i);
        }
    }
    return invalidIDs
}

// part 2
function checkForRepeatedDigits(id: string): boolean {
    let curPointer = 1;
    let curDigits = id[0]!;
    while (curPointer < id.length/2) {
        const remainder = id.slice(curPointer, id.length);
        if (remainder.length % curDigits.length === 0) {
            const expectedMatches =  remainder.length / curDigits.length;
            const regex = new RegExp(curDigits, "gi");;
            const matches = remainder.match(regex)?.length;
            if (matches === expectedMatches) {
                return true
            }
        }
        curDigits += id[curPointer];
        curPointer++;
    }
    
    return false;
}

function main() {
    const ids: string[] = loadFile();
    let totalInvalidIDs = 0;

    ids.map((id => {
       checkRange(id).forEach((invalidID) => {
        totalInvalidIDs += invalidID;
       }) 

    }))


    console.log(totalInvalidIDs);
}


main();