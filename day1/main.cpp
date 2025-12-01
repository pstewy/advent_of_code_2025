#include <vector>
#include <fstream>
#include <string>
#include <iostream>

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

int computeNextValue(int curValue, int move) {
    // TODO: The values can be hire than 99, so we need to account for that
    if (abs(move) > 100) {
        // Taking the mod will give us the actual number we care about
        move = move % 100;
    }

    if (move <= 0) {
        // We are moving left. Does doing so put us below zero?
        if (curValue < abs(move)) {
            int difference = abs(move) - curValue - 1; // sub 1 to account for zero
            return 99 - difference;
        } else {
            return curValue + move;
        }
    } else {
        // We are moving right. Does doing so put us above 99?
        int sum = curValue + move;
        if (sum > 99) {
            return sum - 99 - 1; // sub 1 to account for zero
        } else {
            return sum;
        }
    }
}

int main() {
    vector<int> input = readFile("input_part_1.txt");
    int zeroCounts = 0;
    int curValue = 50;
    for (int i = 0; i < input.size(); i++) {
        int nextMove = input[i];
        int prevVal = curValue;
        curValue = computeNextValue(curValue, nextMove);
        cout <<"curvalue " << prevVal << " moving " << nextMove << " results in " << curValue << "\n";
        if (curValue == 0) {
            zeroCounts++;
        }
    }
    cout << zeroCounts;
}