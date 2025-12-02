#include <vector>
#include <fstream>
#include <string>
#include <iostream>
#include <tuple>

using namespace std;

int parseLine(string l) {
    int length = l.length();
    if (length == 0) {
        return 0;
    }

    char dir = l[0];
    string raw;
    for (int i = 1; i < length; i++) {
        raw += l[i];
    }
    if (dir == 'R') {
        return stoi(raw);
    } else {
        return stoi("-" + raw);
    }
}

vector<int> readFile(string filename) {
    ifstream file(filename);
    string raw;
    vector<int> output;

    while (getline(file, raw))
    {
        output.push_back(parseLine(raw));
    }

    return output;
}

struct nextValCal {
    int nextVal;
    int zeroCount;
};

nextValCal computeNextValue(int curValue, int move) {
    int zeroCount = 0;
    int nextValue = curValue;
    if (move < 0) {
        while (move < 0) {
            nextValue--;
            if (nextValue < 0) {
                nextValue = 99;
            }
            if (nextValue == 0) {
                zeroCount++;
            }
            move++;
            cout << "nextVal " << nextValue << " curMove " << move << "\n";
        }
    } else {
        while (move > 0) {
            nextValue++;
            if (nextValue > 99) {
                nextValue = 0;
            }
            if (nextValue == 0) {
                zeroCount++;
            }
            move--;
        }
    }

    nextValCal valCal;
    valCal.nextVal = nextValue;
    valCal.zeroCount = zeroCount;
    return valCal;
}

int main() {
    vector<int> input = readFile("input.txt");
    int zeroCounts = 0;
    int curValue = 50;
    for (int i = 0; i < input.size(); i++) {
        int nextMove = input[i];
        int prevVal = curValue;
        nextValCal nextVal = computeNextValue(curValue, nextMove);
        curValue = nextVal.nextVal;
        zeroCounts += nextVal.zeroCount;
        cout <<"curvalue " << prevVal << " moving " << nextMove << " results in " << curValue << "\n";
    }
    cout << zeroCounts;
}